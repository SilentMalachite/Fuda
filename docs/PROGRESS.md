# PROGRESS（作業の続きのメモ）

> セッションの始めに、まずこのファイルを読む。作業の区切りごとに書き直す。
> 途中で止まっても、次の人（次のセッションの AI も）がこのファイルだけで続きを始められるように書く。

最終更新: 2026-11-19

## いまの状態（1行）

Phase 0-1a（macOS ビルド・テスト・プロセス起動）完了。手元での画面描画目視と Phase 0-1b（Windows ビルド）は未確認。

## 次にやること（1つだけ）

手元で `cargo run -p fuda-spike` の空ウィンドウ描画を目視確認したうえで、ROADMAP Phase 0-2（BIZ UDPゴシックの `import` と日本語・時刻数字の桁揃え確認）へ進む。

## 最後にやったこと

- Phase 0-1 の敵対的レビューを受け、`docs/ROADMAP.md` の 0-1 を `0-1a (macOS)` / `0-1b (Windows)` に分割。
- `.gitattributes` を追加し、`Cargo.toml` の `[workspace.dependencies]` で `slint` / `slint-build` を `=1.19.0` に同期、`spike/fuda-spike/src/main.rs` に `#![deny(clippy::unwrap_used, clippy::expect_used)]` ガードと `PRAGMA foreign_keys = ON` 疎通テストを追加。
- `--all-features`（`dev-preview`, `dev-mcp`, `dev-clock`）を含めた `fmt` / `clippy` / `test` を確認し、依存ライセンス実測と既知環境問題を `docs/PROGRESS.md` に記録。

## 決まったこと（日付つき。新しいものを上に）

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
  - SPEC §7.3 の `suna`（`#A88443`）× `select`（`#DCE9E6`）がコントラスト比 2.79（3.0 未満）になる点の調整。
  - SPEC §8.5 不変条件 #7（知らせ追加時にカード全体の `reminder_log` を消すと既出知らせが二度出しになる問題）の修正。
  - SPEC §8.3 / §8.5 のボード「できた列」0 個・複数個時の仕様と部分ユニーク制約、および Undo 単位（§6.4）と 300ms 自動保存（§5.8）の統合設計。

## わかっている問題

- エージェントのサンドボックス内で通常の `/Users/hiro/.cargo/bin/cargo` を実行すると、`~/.cargo/bin/rustup-init` の削除試行エラー（`Operation not permitted`）や、間接依存 `convert_case 0.10.0` に同梱された `rust-toolchain.toml`（`channel = "1.83.0"`）への `rustup` 同期エラーが発生する。回避策としてツールチェーン直下（`~/.rustup/toolchains/stable-aarch64-apple-darwin/bin`）を `PATH` の先頭に置いて実行している。
- `0-1a` の空ウィンドウはプロセス起動（エラーなし待機）まで確認済みだが、画面描画の目視確認は未実施（手元で `cargo run -p fuda-spike` にて確認予定）。
- `0-1b`（Windows ビルド）は未確認。

## 測った数字

（Phase 3 以降で、予定の表示・さがす の速さなどを書く）
