//! 同梱フォント（BIZ UDPゴシック）を外部ファイルから起動時に読み込むモジュール。
//!
//! `.slint` の `import "...ttf"` による `include_bytes!` 静的埋め込みを避け、
//! `GPL-3.0-only` の実行バイナリと `SIL OFL 1.1` のフォントファイルを分離する。

use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::Context;
use slint::fontique_012::fontique;

/// UI の `default-font-family` で参照するフォントファミリー名
pub const BUNDLED_FONT_FAMILY: &str = "BIZ UDPGothic";

/// 独立ファイルとして同梱・読込するフォントファイル名一覧
pub const BUNDLED_FONT_FILES: [&str; 2] = ["BIZUDPGothic-Regular.ttf", "BIZUDPGothic-Bold.ttf"];

/// 実行バイナリと開発環境に応じたフォント探索候補ディレクトリを優先順で返す
pub fn candidate_font_dirs(exe_dir: Option<&Path>, manifest_dir: Option<&Path>) -> Vec<PathBuf> {
    let mut dirs = Vec::new();
    if let Some(exe_dir) = exe_dir {
        // macOS バンドル（.app/Contents/MacOS/<bin> -> .app/Contents/Resources/fonts）
        if let Some(contents_dir) = exe_dir.parent() {
            dirs.push(contents_dir.join("Resources").join("fonts"));
        }
        // Windows / Linux / フラット配布（実行ファイルと同じフォルダ配下の fonts）
        dirs.push(exe_dir.join("fonts"));
    }
    if let Some(manifest_dir) = manifest_dir {
        // 開発時（cargo run / cargo test）
        dirs.push(manifest_dir.join("fonts"));
    }
    dirs
}

/// 必要な同梱フォントファイルがすべて揃っている最初のディレクトリを返す
pub fn find_bundled_fonts_dir(
    exe_dir: Option<&Path>,
    manifest_dir: Option<&Path>,
) -> Option<PathBuf> {
    candidate_font_dirs(exe_dir, manifest_dir)
        .into_iter()
        .find(|dir| {
            BUNDLED_FONT_FILES
                .iter()
                .all(|file| dir.join(file).is_file())
        })
}

/// 指定ディレクトリの同梱フォントをすべて `fontique::Collection` へ登録する
pub fn register_fonts_from_dir(
    collection: &mut fontique::Collection,
    fonts_dir: &Path,
) -> anyhow::Result<usize> {
    let mut registered_faces = 0usize;
    for file in BUNDLED_FONT_FILES {
        let path = fonts_dir.join(file);
        let bytes = std::fs::read(&path)
            .with_context(|| format!("フォントファイルの読込に失敗: {}", path.display()))?;
        let registered = collection.register_fonts(fontique::Blob::new(Arc::new(bytes)), None);
        let face_count: usize = registered.iter().map(|(_, infos)| infos.len()).sum();
        if face_count == 0 {
            anyhow::bail!(
                "フォントとして登録できるフェイスがありません: {}",
                path.display()
            );
        }
        registered_faces = registered_faces.saturating_add(face_count);
    }
    if collection.family_id(BUNDLED_FONT_FAMILY).is_none() {
        anyhow::bail!(
            "登録したフォントにファミリー '{}' が見つかりません",
            BUNDLED_FONT_FAMILY
        );
    }
    Ok(registered_faces)
}

/// 同梱フォントディレクトリを探索し、Slint の共有フォントコレクションへ登録する
pub fn register_bundled_fonts() -> anyhow::Result<PathBuf> {
    let exe_path = std::env::current_exe().ok();
    let exe_dir = exe_path.as_deref().and_then(Path::parent);
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    let fonts_dir = find_bundled_fonts_dir(exe_dir, Some(manifest_dir))
        .context("同梱フォントディレクトリ（BIZUDPGothic-Regular/Bold.ttf）が見つかりません")?;
    let mut collection = slint::fontique_012::shared_collection();
    let faces = register_fonts_from_dir(&mut collection, &fonts_dir)?;
    if faces == 0 {
        anyhow::bail!("同梱フォントの登録数が 0 件です: {}", fonts_dir.display());
    }
    Ok(fonts_dir)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    struct TempDirGuard {
        path: PathBuf,
    }

    impl TempDirGuard {
        fn new(prefix: &str) -> Self {
            let unique = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("システム時刻の取得に失敗")
                .as_nanos();
            let path = std::env::temp_dir().join(format!(
                "fuda_fonts_test_{prefix}_{}_{unique}",
                std::process::id()
            ));
            std::fs::create_dir_all(&path).expect("テスト用一時ディレクトリの作成に失敗");
            Self { path }
        }
    }

    impl Drop for TempDirGuard {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.path);
        }
    }

    #[test]
    fn candidate_font_dirs_prioritizes_bundle_then_exe_then_manifest() {
        let exe_dir = Path::new("/tmp/fuda.app/Contents/MacOS");
        let manifest_dir = Path::new("/workspace/spike/fuda-spike");
        let dirs = candidate_font_dirs(Some(exe_dir), Some(manifest_dir));

        assert_eq!(
            dirs,
            vec![
                PathBuf::from("/tmp/fuda.app/Contents/Resources/fonts"),
                PathBuf::from("/tmp/fuda.app/Contents/MacOS/fonts"),
                PathBuf::from("/workspace/spike/fuda-spike/fonts"),
            ]
        );
    }

    #[test]
    fn find_bundled_fonts_dir_skips_incomplete_dir() {
        let temp = TempDirGuard::new("incomplete");
        let exe_dir = temp.path.join("Contents/MacOS");
        let bundle_fonts = temp.path.join("Contents/Resources/fonts");
        let exe_fonts = exe_dir.join("fonts");
        std::fs::create_dir_all(&bundle_fonts).unwrap();
        std::fs::create_dir_all(&exe_fonts).unwrap();

        // 最初の候補（Resources/fonts）は Regular のみ配置して不完全にする
        std::fs::write(bundle_fonts.join(BUNDLED_FONT_FILES[0]), b"dummy").unwrap();
        // 次の候補（exe_dir/fonts）には両方のファイルを配置する
        for file in BUNDLED_FONT_FILES {
            std::fs::write(exe_fonts.join(file), b"dummy").unwrap();
        }

        let found = find_bundled_fonts_dir(Some(&exe_dir), None);
        assert_eq!(found, Some(exe_fonts));
    }

    #[test]
    fn register_fonts_from_dir_registers_regular_and_bold_biz_udp_gothic() {
        let fonts_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("fonts");
        let mut collection = fontique::Collection::new(fontique::CollectionOptions {
            system_fonts: false,
            ..Default::default()
        });

        let faces = register_fonts_from_dir(&mut collection, &fonts_dir)
            .expect("同梱 BIZ UDPゴシックの登録に失敗");
        assert_eq!(faces, 2);

        let family_id = collection
            .family_id(BUNDLED_FONT_FAMILY)
            .expect("BIZ UDPGothic ファミリーが登録されていません");
        let family = collection
            .family(family_id)
            .expect("BIZ UDPGothic のファミリー情報が取得できません");
        let weights: Vec<fontique::FontWeight> =
            family.fonts().iter().map(|info| info.weight()).collect();
        assert!(
            weights.contains(&fontique::FontWeight::NORMAL),
            "Regular (400) が登録されていません: {weights:?}"
        );
        assert!(
            weights.contains(&fontique::FontWeight::BOLD),
            "Bold (700) が登録されていません: {weights:?}"
        );
    }

    #[test]
    fn main_slint_does_not_embed_ttf_fonts() {
        let main_slint = include_str!("../ui/main.slint");
        assert!(
            !main_slint.contains(".ttf"),
            "ui/main.slint に .ttf の静的 import が残っています"
        );
    }
}
