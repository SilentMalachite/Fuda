# PROGRESS（作業の続きのメモ）

> セッションの始めに、まずこのファイルを読む。作業の区切りごとに書き直す。
> 途中で止まっても、次の人（次のセッションの AI も）がこのファイルだけで続きを始められるように書く。

最終更新: 2026-11-20

## いまの状態（1行）

Phase 0-4 完了・敵対的レビュー（C-1〜C-3 / H-1〜H-4 / M-1〜M-11）の仕様修正およびスパイク修正反映済み。次は Phase 0-5（`DragArea` / `DropArea` の確認）。

## 次にやること（1つだけ）

ROADMAP Phase 0-5（`DragArea` / `DropArea` で 2 列間のカード移動と置く位置の線表示）を `spike/fuda-spike` に実装して確認する。

## 最後にやったこと

- 敵対的レビューの残り High（H-1〜H-4）および Medium（M-1〜M-11）を `docs/SPEC.md`・`docs/ROADMAP.md` に反映（`BSL-1.0`/`Unlicense` 許可リストと C-4 は引き続き要検討として残置）。
- H-1 / H-4 に連動して `spike/fuda-spike/` を修正（`border-control` / `danger` / `on-danger` 追加、余白の固定論理 px 化、3px フォーカス枠と選択下線の重なり解消、`PlainTimeRow` の `wrap: word-wrap`、全 `Text` への `line-height-factor` 適用、`LineEdit`/`TextEdit` の `accessible-label` 追加、`RefCell::borrow_mut()` 解放後 UI 更新パターン化、`\r\n` 正規化＋テスト追加）。

## 決まったこと（日付つき。新しいものを上に）

- 2026-11-20 Medium（M-1〜M-11）仕様確定: 予定の同時刻順は `created_at ASC, id ASC`、やること並び替えは `place = 'tasks'` 全体の `position` 振り直し、繰り返し起点日の「この回から先をやめる」はゴミ箱へ移す（曜日最低 1 つ必須）、入力欄の Undo（300ms 自動保存と連動＋テキスト入力欄内 `Ctrl/⌘+Z` 優先）、SQLite `CHECK` に `date(col) IS col`・厳格 `weekdays`・`strftime` 採用、さがすは空白区切り AND 検索、0:00 日付またぎは状態バー案内、時刻入力の午前/午後・桁数・区切り境界確定、状態バーの閉じる表記は［閉じる］（行・列の「…」は `accessible-label` 必須の例外）、`focus.card_id` 自動解除の Undo 復元および予定画面でのチェック済み表示維持を確定。
- 2026-11-20 H-4（スパイク品質・レイアウト修正）: `Theme` の余白は SPEC §7.1 どおり固定論理 px（`4/8/12/16/24/32`）とし、3px フォーカス枠の内側に選択下線を配置、コールバックでは `ImeViewSnapshot` を取り出して `RefCell` 借用解放後に Slint を更新、複数行文字数カウントは `\r\n` を `\n` に正規化する方式へ統一。
- 2026-11-20 H-3（バックアップ命名・復元安全化）: バックアップは `backups/fuda-YYYY-MM-DD-HHMMSS.sqlite3`＋`VACUUM INTO ?1` パラメータバインド、復元は設定画面内の 2 段階インライン操作とし、事前整合チェック・復元前バックアップ・接続クローズ・`-wal`/`-shm` 削除・一時ファイル置換の手順を確定。
- 2026-11-20 H-2（知らせアルゴリズム・不変条件 #7）: `reminder_log` の `state IN ('showing', 'again')` は 24時間窓外でも別経路で取得して「10分後にもう一度」の消失を防止、スリープ復帰は `last_checked_at` で判定、表示中に完了・移動された場合は即消去、同一条件・同時刻の重複知らせを DB 部分ユニーク制約と `service` で禁止し、知らせ追加時は他ログを消さないよう不変条件 #7 を改訂。
- 2026-11-20 H-1（WCAG 配色・トークン追加）: 操作要素用枠線 `border-control`（やさしい `#7D7A72` / くらい `#8E9C98` / はっきり `#000000`）と `danger` / `on-danger` を追加し、`hakkiri` の `accent` を `#083E7D`、カード色 `suna`（やさしい）を `#9E7B3A` に調整して全組合わせのコントラスト基準を満たすよう改訂。

- 2026-11-19 C-3（知らせの窓の到達性と配置）: 独立ウィンドウ間の `Tab` 移動仕様（旧 §6.1）を改訂し、知らせ表示中はメインウィンドウ上部にも「いま出ている知らせ」欄（Tab 順・`Ctrl/⌘+Shift+R` 到達）を同期表示するとともに、別窓は対象ディスプレイの作業領域（16px 余白、高さ上限 60%・スクロール＋［全部わかった］固定）へ配置する方針を確定。
- 2026-11-19 C-2（二重起動防止）: `fuda.lock` の OS 排他ロック（`std::fs::File::try_lock`）とローカル IPC（ネットワークソケット非使用、macOS/Linux は `fuda.ipc`、Windows は Named Pipe 等）で既存ウィンドウを手前に出し、DB 二重オープンと知らせ重複を防ぐ方針を確定（ROADMAP `0-8b` / `4-7b`）。
- 2026-11-19 C-1（GPLv3 × SIL OFL 1.1 分離）: `.slint` の `.ttf` 静的 `import`（`include_bytes!` 埋め込み）を廃止し、`BIZUDPGothic-Regular/Bold.ttf` と `OFL.txt` を独立ファイル（macOS `.app/Contents/Resources/fonts/`、Windows `<exe_dir>/fonts/`）として同梱して起動時に `slint::fontique_012::shared_collection()` 経由で読み込む方針を確定・`spike/fuda-spike` へ実装。
- 2026-11-19 ライセンスを GNU GPLv3（`GPL-3.0-only`）に統一して確定（Slint に LGPL 版はなく、オープンソース向けライセンス `GPL-3.0-only` にそろえる。`LICENSE`・`Cargo.toml`・`README.md`・`README.ja.md`・`SPEC.md` §11/§13 を整備済み）。
- 2026-11-19 Slint 1.19.0 は Text の line-height-factor をサポート。BIZ UDPゴシック（Regular/Bold とも unitsPerEm=2048, asc=1802, desc=-246, gap=0）の natural line height は厳密に 1.0000em のため、line-height-factor: 1.6 で SPEC §7.1 の目安 1.6 倍になる。
- 2026-11-19 Slint 1.19.0 の LineEdit / TextEdit は macOS の日本語 IME で変換中表示・変換確定 Enter 時の accepted 非発火・複数行改行とも正常動作を確認（Windows 確認は 0-3b として別途実施）。
- 2026-11-19 BIZ UDPゴシック（`googlefonts/morisawa-biz-ud-gothic` Version 1.051、SIL OFL 1.1）は `default-font-family: "BIZ UDPGothic"` で Regular(400) / Bold(700) とも参照可能。数字 `1` のみ幅が狭く（`1290` vs `1556/2048`）`tnum` も非搭載（Slint 1.19 側も OpenType feature 指定なし）のため、SPEC §7.4 に従い時刻欄を固定幅にして桁を揃える。
- 2026-11-19 `Cargo.toml` の `[workspace.dependencies]` で主要クレートを一元管理し、`slint` と `slint-build` は `=1.19.0` で完全一致させる。
- 2026-11-19 `rust-toolchain.toml` は当面 `channel = "stable"`（現環境 `rustc 1.100.0`、edition 2024）とし、Windows / CI 導入時に具体バージョン固定へ切り替える。
- 2026-11-19 依存追加（`spike/fuda-spike`）: `slint` 1.19.0 / `slint-build` 1.19.0 — UI フレームワークと `.slint` ビルド（実測 SPDX: `GPL-3.0-only OR LicenseRef-Slint-Royalty-free-2.0 OR LicenseRef-Slint-Software-3.0`）。
- 2026-11-19 依存追加（`spike/fuda-spike`）: `rusqlite` 0.43.0（`features = ["bundled"]`）— 同梱 SQLite（v3.54.1）の読み書き（MIT）。
- 2026-11-19 依存追加（`spike/fuda-spike`）: `anyhow` 1.0.104 — アプリ・スパイク側のエラー処理（MIT OR Apache-2.0）。
- 2026-11-19 Phase 0 の Windows 確認項目は macOS 先行で実装・検証し、Windows 固有の確認は別途 Windows 環境で行う。
- 2026-11-19 知らせは「知らせの窓」（いつも手前・フォーカスを奪わない・消すまで残る）が主。パソコンの通知は補助。音は MVP では作らない（P2、既定オフ）。
- 2026-11-19 知らせは fuda が動いているあいだだけ。ウィンドウを閉じてもトレイでうしろで動き続けるのを既定にする。閉じているあいだの知らせは、次に動いたとき 24時間ぶんまで出す。
- 2026-11-19 予定は5つ目の場所。予定の画面（スケジュール帳）は1日・1週間・1か月。日付のある やること・ボード のカードも、その日に見せる（置き場所は変えない）。
- 2026-11-19 予定の繰り返しは MVP（毎日／毎週／2週間ごと／毎月／毎年、この回だけ休み）。やることの繰り返しは P2。
- 2026-11-19 祝日は内閣府の CSV を同梱（毎年更新）。土日・祝日は色でなく文字で示す。
- 2026-11-19 アプリ名は **fuda**。
- 2026-11-19 扱うのは「カード」1種類。カードはいつも1か所にある。
- 2026-11-19 Slint には LGPL 版がない。Royalty-free License 2.0 を既定案にする（SPEC §11.1。Phase 0 のあとで最終決定）。
- 2026-11-19 フォントは BIZ UDPゴシックを同梱する。
- 2026-11-19 音・アニメーションは既定でオフ。確認ダイアログは「ゴミ箱を空にする」だけ。

## まだ決まっていないこと・質問

- SPEC §13 を見る。
- 敵対的レビューで判明した要検討事項（SPEC 反映・Phase 1 までに判断）:
  - `slint 1.19.0` の間接依存に含まれる `BSL-1.0`（`clipboard-win`, `error-code`）と `Unlicense`（`ksni`）を SPEC §11.2 / `deny.toml` の許可リストへ追加するか。
  - （C-4）SPEC §8.3 / §8.5 のボード削除・列削除時のゴミ箱／しまったカードを含む `column_id`（`ON DELETE RESTRICT`）と Undo 設計、最後の 1 ボード削除可否、および `is_done_column`（0 個・複数個）の部分ユニーク制約の整理。

## わかっている問題

- エージェントのサンドボックス内で通常の `/Users/hiro/.cargo/bin/cargo` を実行すると、`~/.cargo/bin/rustup-init` の削除試行エラー（`Operation not permitted`）や、間接依存 `convert_case 0.10.0` に同梱された `rust-toolchain.toml`（`channel = "1.83.0"`）への `rustup` 同期エラーが発生する。回避策としてツールチェーン直下（`~/.rustup/toolchains/stable-aarch64-apple-darwin/bin`）を `PATH` の先頭に置いて実行している。
- `0-3b`（Windows 日本語入力確認）は未確認。
- `0-1b`（Windows ビルド）は未確認。

## 測った数字

（Phase 3 以降で、予定の表示・さがす の速さなどを書く）
