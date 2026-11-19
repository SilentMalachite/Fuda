# PROGRESS（作業の続きのメモ）

> セッションの始めに、まずこのファイルを読む。作業の区切りごとに書き直す。
> 途中で止まっても、次の人（次のセッションの AI も）がこのファイルだけで続きを始められるように書く。

最終更新: 2026-11-19

## いまの状態（1行）

Phase 0-4 完了（macOS 目視確認済み）。次は Phase 0-5（`DragArea` / `DropArea` の確認）。

## 次にやること（1つだけ）

ROADMAP Phase 0-5（`DragArea` / `DropArea` で 2 列間のカード移動と置く位置の線表示）を `spike/fuda-spike` に実装して確認する。

## 最後にやったこと

- `palette.slint` を `theme.slint`（`global Theme`）へ統合し、`font-base` 連動の `font-heading` / `font-small` / `space-*` / `time-range-width` と `line-height: 1.6` を定義。
- `phase04_theme.slint`（16/18/22/26px 切替、カード見本、`line-height-factor` 1.0 vs 1.6 左右比較）と `main.rs` の TTF 垂直メトリクス検証テストを追加し、macOS での目視確認まで完了。

## 決まったこと（日付つき。新しいものを上に）

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
  - SPEC §7.3 の `suna`（`#A88443`）× `select`（`#DCE9E6`）がコントラスト比 2.79（3.0 未満）になる点の調整。
  - SPEC §8.5 不変条件 #7（知らせ追加時にカード全体の `reminder_log` を消すと既出知らせが二度出しになる問題）の修正。
  - SPEC §8.3 / §8.5 のボード「できた列」0 個・複数個時の仕様と部分ユニーク制約、および Undo 単位（§6.4）と 300ms 自動保存（§5.8）の統合設計。

## わかっている問題

- エージェントのサンドボックス内で通常の `/Users/hiro/.cargo/bin/cargo` を実行すると、`~/.cargo/bin/rustup-init` の削除試行エラー（`Operation not permitted`）や、間接依存 `convert_case 0.10.0` に同梱された `rust-toolchain.toml`（`channel = "1.83.0"`）への `rustup` 同期エラーが発生する。回避策としてツールチェーン直下（`~/.rustup/toolchains/stable-aarch64-apple-darwin/bin`）を `PATH` の先頭に置いて実行している。
- `0-3b`（Windows 日本語入力確認）は未確認。
- `0-1b`（Windows ビルド）は未確認。

## 測った数字

（Phase 3 以降で、予定の表示・さがす の速さなどを書く）
