#![deny(clippy::unwrap_used, clippy::expect_used)]

// Slint が生成するモジュールコード内部では .unwrap() が使われるため生成領域のみ許可する
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod ui {
    slint::include_modules!();
}

use slint::ComponentHandle;
use ui::SpikeWindow;

/// メモリ上の SQLite 接続を開いて SQLite のバージョンを取得する
fn sqlite_version() -> anyhow::Result<String> {
    let conn = rusqlite::Connection::open_in_memory()?;
    let version: String = conn.query_row("SELECT sqlite_version()", [], |row| row.get(0))?;
    Ok(version)
}

fn main() -> anyhow::Result<()> {
    let version = sqlite_version()?;
    println!("SQLite version: {version}");

    let ui = SpikeWindow::new()?;
    ui.run()?;
    Ok(())
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    #[test]
    fn sqlite_version_returns_v3() {
        let version = sqlite_version().expect("メモリ上の SQLite バージョン取得に失敗");
        assert!(
            version.starts_with("3."),
            "想定外の SQLite バージョン: {version}"
        );
    }

    #[test]
    fn sqlite_in_memory_foreign_keys_and_crud_work() {
        let conn = rusqlite::Connection::open_in_memory().expect("メモリDBの作成に失敗");
        conn.execute_batch(
            "PRAGMA foreign_keys = ON;
             CREATE TABLE parent (id INTEGER PRIMARY KEY);
             CREATE TABLE child (
                 id INTEGER PRIMARY KEY,
                 parent_id INTEGER NOT NULL REFERENCES parent(id)
             );",
        )
        .expect("テスト用テーブルの作成に失敗");

        conn.execute("INSERT INTO parent (id) VALUES (1)", [])
            .expect("親行の挿入に失敗");
        conn.execute("INSERT INTO child (id, parent_id) VALUES (10, 1)", [])
            .expect("子行の挿入に失敗");

        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM child WHERE parent_id = 1", [], |r| {
                r.get(0)
            })
            .expect("子行の取得に失敗");
        assert_eq!(count, 1);

        // 外部キー制約が有効になっていることを確認する
        let fk_err = conn.execute("INSERT INTO child (id, parent_id) VALUES (11, 999)", []);
        assert!(fk_err.is_err(), "foreign_keys = ON なのに不正挿入が通った");
    }

    #[test]
    fn bundled_biz_udp_gothic_fonts_and_license_exist() {
        let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let fonts_dir = manifest_dir.join("fonts");

        for font_file in ["BIZUDPGothic-Regular.ttf", "BIZUDPGothic-Bold.ttf"] {
            let path = fonts_dir.join(font_file);
            let bytes = std::fs::read(&path)
                .unwrap_or_else(|e| panic!("フォントファイル読込失敗 ({}): {e}", path.display()));
            assert!(
                bytes.len() > 4,
                "フォントファイルが空または小さすぎます: {}",
                path.display()
            );
            assert_eq!(
                &bytes[..4],
                &[0x00, 0x01, 0x00, 0x00],
                "TrueType マジックバイトが一致しません: {}",
                path.display()
            );
        }

        let ofl_path = fonts_dir.join("OFL.txt");
        let ofl_text = std::fs::read_to_string(&ofl_path)
            .unwrap_or_else(|e| panic!("OFL.txt 読込失敗 ({}): {e}", ofl_path.display()));
        assert!(
            ofl_text.contains("SIL OPEN FONT LICENSE Version 1.1"),
            "OFL.txt に SIL OPEN FONT LICENSE Version 1.1 が含まれていません"
        );
    }
}
