#![deny(clippy::unwrap_used, clippy::expect_used)]

mod fonts;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

// Slint が生成するモジュールコード内部では .unwrap() が使われるため生成領域のみ許可する
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod ui {
    slint::include_modules!();
}

use slint::ComponentHandle;
use ui::SpikeWindow;

const MAX_ADDED_CARDS: usize = 10;
const MAX_EVENT_LOGS: usize = 12;

/// Phase 0-3 の日本語入力確認でイベント回数・追加一覧・観測ログを保持する状態
#[derive(Debug, Default)]
struct ImeSpikeState {
    seq: usize,
    accepted_count: i32,
    line_edited_count: i32,
    added_cards: Vec<String>,
    logs: VecDeque<String>,
}

impl ImeSpikeState {
    /// シーケンス番号を進めて先頭にログを追記し、最大件数に丸める
    fn push_log(&mut self, message: impl FnOnce(usize) -> String) -> String {
        self.seq = self.seq.saturating_add(1);
        let line = message(self.seq);
        self.logs.push_front(line.clone());
        self.logs.truncate(MAX_EVENT_LOGS);
        line
    }

    /// 1行入力の accepted コールバックを記録し、空白でなければカードを追加する
    fn on_line_accepted(&mut self, text: &str) -> (String, bool) {
        self.accepted_count = self.accepted_count.saturating_add(1);
        let trimmed = text.trim();
        let should_clear = if trimmed.is_empty() {
            false
        } else {
            self.added_cards.insert(0, trimmed.to_string());
            self.added_cards.truncate(MAX_ADDED_CARDS);
            true
        };
        let log = self.push_log(|seq| format!("#{seq} [LineEdit] accepted: \"{text}\""));
        (log, should_clear)
    }

    /// 1行入力の edited コールバックを記録する
    fn on_line_edited(&mut self, text: &str) -> String {
        self.line_edited_count = self.line_edited_count.saturating_add(1);
        self.push_log(|seq| format!("#{seq} [LineEdit] edited: \"{text}\""))
    }

    /// キー押下イベントを記録する
    fn on_key_pressed(&mut self, source: &str, key: &str, has_modifier: bool) -> String {
        self.push_log(|seq| {
            format!("#{seq} [{source}] key-pressed: {key} (modifier={has_modifier})")
        })
    }

    /// 複数行入力の edited コールバックから行数・文字数を計算し記録する
    fn on_multi_edited(&mut self, text: &str) -> (i32, i32, String) {
        // Windows の CRLF 改行で `\r` を 1 文字と数えないよう正規化する
        let normalized = text.replace("\r\n", "\n");
        let lines = i32::try_from(normalized.split('\n').count()).unwrap_or(i32::MAX);
        let chars = i32::try_from(normalized.chars().count()).unwrap_or(i32::MAX);
        let log =
            self.push_log(|seq| format!("#{seq} [TextEdit] edited (lines={lines}, chars={chars})"));
        (lines, chars, log)
    }

    /// カウンタ・カード一覧・ログをすべて初期化する
    fn clear(&mut self) {
        self.seq = 0;
        self.accepted_count = 0;
        self.line_edited_count = 0;
        self.added_cards.clear();
        self.logs.clear();
    }

    /// 追加されたカード一覧を番号付き文字列へ整形する
    fn added_cards_text(&self) -> String {
        self.added_cards
            .iter()
            .enumerate()
            .map(|(idx, title)| format!("{}. {title}", idx + 1))
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// イベントログを改行区切り文字列へ整形する
    fn event_log_text(&self) -> String {
        self.logs.iter().cloned().collect::<Vec<_>>().join("\n")
    }

    /// RefCell の借用解放後に UI へ反映するためのスナップショットを作成する
    fn snapshot(&self) -> ImeViewSnapshot {
        ImeViewSnapshot {
            accepted_count: self.accepted_count,
            line_edited_count: self.line_edited_count,
            added_cards_text: self.added_cards_text(),
            event_log_text: self.event_log_text(),
        }
    }
}

/// RefCell 借用ガードを持たないまま Slint プロパティを更新するための表示用スナップショット
struct ImeViewSnapshot {
    accepted_count: i32,
    line_edited_count: i32,
    added_cards_text: String,
    event_log_text: String,
}

/// メモリ上の SQLite 接続を開いて SQLite のバージョンを取得する
fn sqlite_version() -> anyhow::Result<String> {
    let conn = rusqlite::Connection::open_in_memory()?;
    let version: String = conn.query_row("SELECT sqlite_version()", [], |row| row.get(0))?;
    Ok(version)
}

/// `ImeViewSnapshot` の表示用プロパティをウィンドウへ反映する
fn sync_ime_view(ui: &SpikeWindow, snapshot: &ImeViewSnapshot) {
    ui.set_accepted_count(snapshot.accepted_count);
    ui.set_line_edited_count(snapshot.line_edited_count);
    ui.set_added_cards_text(snapshot.added_cards_text.as_str().into());
    ui.set_event_log_text(snapshot.event_log_text.as_str().into());
}

fn main() -> anyhow::Result<()> {
    let version = sqlite_version()?;
    println!("SQLite version: {version}");

    match fonts::register_bundled_fonts() {
        Ok(dir) => println!("Bundled fonts loaded from: {}", dir.display()),
        Err(err) => {
            eprintln!("警告: 同梱フォントを読み込めず既定フォントにフォールバックします: {err}")
        }
    }

    let ui = SpikeWindow::new()?;
    let state = Rc::new(RefCell::new(ImeSpikeState::default()));

    {
        let weak = ui.as_weak();
        let state = Rc::clone(&state);
        ui.on_line_accepted(move |text| {
            // Slint 側コールバックの再入による BorrowMutError を防ぐため、UI 更新前に借用を解放する
            let (log, should_clear, snapshot) = {
                let mut st = state.borrow_mut();
                let (log, should_clear) = st.on_line_accepted(text.as_str());
                (log, should_clear, st.snapshot())
            };
            println!("{log}");
            if let Some(window) = weak.upgrade() {
                if should_clear {
                    window.set_line_text("".into());
                }
                sync_ime_view(&window, &snapshot);
            }
        });
    }

    {
        let weak = ui.as_weak();
        let state = Rc::clone(&state);
        ui.on_line_edited(move |text| {
            let (log, snapshot) = {
                let mut st = state.borrow_mut();
                let log = st.on_line_edited(text.as_str());
                (log, st.snapshot())
            };
            println!("{log}");
            if let Some(window) = weak.upgrade() {
                sync_ime_view(&window, &snapshot);
            }
        });
    }

    {
        let weak = ui.as_weak();
        let state = Rc::clone(&state);
        ui.on_line_key_pressed(move |key, has_modifier| {
            let (log, snapshot) = {
                let mut st = state.borrow_mut();
                let log = st.on_key_pressed("LineEdit", key.as_str(), has_modifier);
                (log, st.snapshot())
            };
            println!("{log}");
            if let Some(window) = weak.upgrade() {
                sync_ime_view(&window, &snapshot);
            }
        });
    }

    {
        let weak = ui.as_weak();
        let state = Rc::clone(&state);
        ui.on_multi_edited(move |text| {
            let (lines, chars, log, snapshot) = {
                let mut st = state.borrow_mut();
                let (lines, chars, log) = st.on_multi_edited(text.as_str());
                (lines, chars, log, st.snapshot())
            };
            println!("{log}");
            if let Some(window) = weak.upgrade() {
                window.set_multi_line_count(lines);
                window.set_multi_char_count(chars);
                sync_ime_view(&window, &snapshot);
            }
        });
    }

    {
        let weak = ui.as_weak();
        let state = Rc::clone(&state);
        ui.on_multi_key_pressed(move |key, has_modifier| {
            let (log, snapshot) = {
                let mut st = state.borrow_mut();
                let log = st.on_key_pressed("TextEdit", key.as_str(), has_modifier);
                (log, st.snapshot())
            };
            println!("{log}");
            if let Some(window) = weak.upgrade() {
                sync_ime_view(&window, &snapshot);
            }
        });
    }

    {
        let weak = ui.as_weak();
        let state = Rc::clone(&state);
        ui.on_clear_all(move || {
            let snapshot = {
                let mut st = state.borrow_mut();
                st.clear();
                st.snapshot()
            };
            println!("[clear] ログと一覧を初期化しました");
            if let Some(window) = weak.upgrade() {
                sync_ime_view(&window, &snapshot);
            }
        });
    }

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

    #[test]
    fn ime_state_records_accepted_and_adds_non_empty_card() {
        let mut state = ImeSpikeState::default();

        // 空白のみの入力ではカード一覧に追加されないが、accepted 回数とログは記録される
        let (empty_log, should_clear_empty) = state.on_line_accepted("   ");
        assert!(!should_clear_empty);
        assert_eq!(state.accepted_count, 1);
        assert!(state.added_cards_text().is_empty());
        assert!(empty_log.contains("[LineEdit] accepted"));

        // 通常の日本語入力はカード一覧に追加され、クリアフラグが true になる
        let (_log, should_clear) = state.on_line_accepted("  歯医者の予約  ");
        assert!(should_clear);
        assert_eq!(state.accepted_count, 2);
        assert_eq!(state.added_cards_text(), "1. 歯医者の予約");

        // ログ上限 12 件を超えた場合、古いログから押し出されて最新 12 件だけ残る
        for i in 1..=15 {
            state.on_line_edited(&format!("候補{i}"));
        }
        assert_eq!(state.line_edited_count, 15);
        assert_eq!(state.logs.len(), MAX_EVENT_LOGS);
        let log_text = state.event_log_text();
        assert!(log_text.contains("候補15"));
        assert!(!log_text.contains("候補1\""));
    }

    #[test]
    fn ime_state_counts_multiline_japanese_text() {
        let mut state = ImeSpikeState::default();

        let (lines, chars, _) = state.on_multi_edited("明日の予定");
        assert_eq!(lines, 1);
        assert_eq!(chars, 5);

        let (lines, chars, _) = state.on_multi_edited("１行目：歯医者\n２行目：買い物\n");
        assert_eq!(lines, 3);
        assert_eq!(chars, 16);

        let (lines, chars, _) = state.on_multi_edited("1行目\r\n2行目");
        assert_eq!(lines, 2);
        assert_eq!(chars, 7);

        state.on_key_pressed("TextEdit", "Return", false);
        assert!(!state.event_log_text().is_empty());

        state.clear();
        assert_eq!(state.seq, 0);
        assert_eq!(state.accepted_count, 0);
        assert_eq!(state.line_edited_count, 0);
        assert!(state.added_cards_text().is_empty());
        assert!(state.event_log_text().is_empty());
    }

    fn ttf_table_offset(bytes: &[u8], tag: &[u8; 4]) -> usize {
        let num_tables = usize::from(u16::from_be_bytes([bytes[4], bytes[5]]));
        for i in 0..num_tables {
            let rec = 12 + i * 16;
            if &bytes[rec..rec + 4] == tag {
                return u32::from_be_bytes([
                    bytes[rec + 8],
                    bytes[rec + 9],
                    bytes[rec + 10],
                    bytes[rec + 11],
                ]) as usize;
            }
        }
        panic!(
            "TTF テーブルが見つかりません: {}",
            String::from_utf8_lossy(tag)
        );
    }

    #[test]
    fn bundled_biz_udp_gothic_vertical_metrics_are_1em() {
        let fonts_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("fonts");

        for font_file in ["BIZUDPGothic-Regular.ttf", "BIZUDPGothic-Bold.ttf"] {
            let path = fonts_dir.join(font_file);
            let bytes = std::fs::read(&path)
                .unwrap_or_else(|e| panic!("フォント読込失敗 ({}): {e}", path.display()));

            let head = ttf_table_offset(&bytes, b"head");
            let units_per_em = u16::from_be_bytes([bytes[head + 18], bytes[head + 19]]);

            let hhea = ttf_table_offset(&bytes, b"hhea");
            let ascender = i16::from_be_bytes([bytes[hhea + 4], bytes[hhea + 5]]);
            let descender = i16::from_be_bytes([bytes[hhea + 6], bytes[hhea + 7]]);
            let line_gap = i16::from_be_bytes([bytes[hhea + 8], bytes[hhea + 9]]);

            let natural_line_height =
                i32::from(ascender) - i32::from(descender) + i32::from(line_gap);
            assert_eq!(
                natural_line_height,
                i32::from(units_per_em),
                "{font_file} の natural line height が 1.0em ではありません"
            );
        }
    }
}
