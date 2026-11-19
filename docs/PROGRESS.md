# PROGRESS（作業の続きのメモ）

> セッションの始めに、まずこのファイルを読む。作業の区切りごとに書き直す。
> 途中で止まっても、次の人（次のセッションの AI も）がこのファイルだけで続きを始められるように書く。

最終更新: 2026-11-19

## いまの状態（1行）

Phase 0-1（macOS）完了。workspace と `spike/fuda-spike` で Slint 1.19.0 + rusqlite 0.43.0（bundled / SQLite 3.54.1）の空ウィンドウ起動を確認。

## 次にやること（1つだけ）

ROADMAP Phase 0-2: BIZ UDPゴシックを `import` して日本語（ひらがな・カタカナ・漢字・記号「〜・※」）を表示し、時刻の数字の桁がそろうか確認する。

## 最後にやったこと

- Git リポジトリ初期化（`main`）、`.gitignore`・`rust-toolchain.toml` 作成、`origin`（`https://github.com/SilentMalachite/Fuda.git`）設定。
- ルート `Cargo.toml`（workspace）と `spike/fuda-spike`（Slint 1.19.0 / slint-build 1.19.0 / rusqlite 0.43.0 / anyhow 1.0.104）を作成し、`Cargo.lock` を生成。
- macOS（rustc 1.100.0）で `cargo fmt`、`cargo clippy -- -D warnings`、`cargo test`（SQLite 3.54.1 確認）、`cargo run -p fuda-spike`（空ウィンドウ起動）を確認。

## 決まったこと（日付つき。新しいものを上に）

- 2026-11-19 依存追加（`spike/fuda-spike`）: `slint` 1.19.0 / `slint-build` 1.19.0 — UI フレームワークと `.slint` ビルド（SPEC §11 に従いライセンス確認対象）。
- 2026-11-19 依存追加（`spike/fuda-spike`）: `rusqlite` 0.43.0（`features = ["bundled"]`）— 同梱 SQLite（v3.54.1）の読み書き（MIT）。
- 2026-11-19 依存追加（`spike/fuda-spike`）: `anyhow` 1.0.104 — アプリ・スパイク側のエラー処理（MIT OR Apache-2.0）。
- 2026-11-19 `rust-toolchain.toml` は `channel = "stable"`（現環境 `rustc 1.100.0`、edition 2024）を指定し、`rustfmt`・`clippy` を同梱する。
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

## わかっている問題

- なし

## 測った数字

（Phase 3 以降で、予定の表示・さがす の速さなどを書く）
