# CLAUDE.md

このリポジトリで作業するときの決まりです。Claude Code だけでなく、ほかの AI エージェントも同じ決まりに従います。

| 知りたいこと | 読むファイル |
|---|---|
| 何を作るか（仕様） | `docs/SPEC.md` |
| どの順番で作るか | `docs/ROADMAP.md` |
| どこまで進んだか・次に何をするか | `docs/PROGRESS.md` |

---

## このアプリ

- **fuda**: ASD・ADHD・聴覚障害などがある人が、疲れずに使える「やること・予定（スケジュール帳）・メモ・ボード（カンバン）」のデスクトップアプリ。決めた時刻に **画面で** 知らせる。
- 扱うのは **カード1種類だけ**。カードはいつも「とりあえず／やること／予定／メモ／ボード」の **どれか1か所** にある。
- 予定の画面はスケジュール帳。日付のあるカードを、場所をまたいで日付ごとに **見せる** だけで、置き場所は変えない。
- Rust + Slint + SQLite。ネットワークを使わず、パソコンの中だけで動く。

---

## 作業の始め方・終わり方

1. **始め**: `docs/PROGRESS.md` を読む → 「次にやること」を1つ確かめる → `docs/ROADMAP.md` のそのタスクと、`docs/SPEC.md` の関係する節を読む。
2. **作業**: 1回に1タスク。1タスク = 1コミット。大きすぎたら分けて、ROADMAP に書き足す。
3. **区切り**: コミットの前に `docs/PROGRESS.md` を書き直す（いまの状態／最後にやったこと／次にやること1つ／決まったこと）。
   - 途中で止まっても、次のセッションが PROGRESS.md だけで再開できるように書く。

---

## 必ず守る UX の決まり（SPEC §2 の要約）

迷ったらこれを優先します。機能とぶつかったら、**機能のほうを削るか後回しにします**。

1. **1画面に目的は1つ**。主ボタンも1つ。見出しで、いまいる場所を示す。
2. **覚えさせない**。自動保存（保存ボタンを作らない）。開き直すと前と同じ画面・同じカードに戻る。次の知らせをいつも見せる。
3. **音に頼らない**。音は既定オフ。知らせは **知らせの窓**（いつも手前・消すまで残る）で出す。
4. **勝手に動かない**。操作なしに並び・レイアウト・スクロール位置を変えない。アニメーションは既定オフ（オンでも 150ms 以下）。**知らせの窓はキーボードのフォーカスを奪わない**。
5. **時間で急かさない**。自動で消えるメッセージ、時間制限、秒のカウントダウンを作らない。残り時間は分単位。
6. **責めない**。日付が過ぎたものを赤・警告で出さない。未完了数のバッジ、連続記録、ポイントを作らない。
7. **取り消せる**。削除はゴミ箱へ。データを変える操作は全部「元に戻す」できる。確認ダイアログは「ゴミ箱を空にする」だけ。
8. **選択肢を少なく**。よく使うものだけ見せ、残りは「その他」へ。設定を足す前に、よい既定値で済まないか考える。
9. **言葉は用語集どおり**（SPEC §3）。アイコンだけのボタンを作らない。
10. **刺激を抑える**。透明・ぼかし・点滅なし（1秒に3回を超える点滅は禁止）。色だけで意味を伝えない（土日・祝日も文字で）。

---

## 技術スタック

| 用途 | 使うもの | 補足 |
|---|---|---|
| UI | `slint` 1.19 系、`slint-build` | ライセンスは SPEC §11（LGPL 版はない。GNU GPLv3（`GPL-3.0-only`）を採用） |
| DB | `rusqlite`（features = `["bundled"]`） | SQLite を一緒にビルドする。WAL |
| 日付・時刻 | `chrono` | 「今日」「いま」は `Clock` トレイトから取る（テストで差し替える） |
| エラー | `thiserror`（core）、`anyhow`（app） | |
| 書き出し | `serde`、`serde_json` | |
| さがす の正規化 | `unicode-normalization` | SPEC §8.6 |
| 保存場所 | `directories` | |
| ログ | `tracing`、`tracing-subscriber` | |
| パソコンの通知 | 候補 `notify-rust` | Phase 0（0-9）で確かめてから決める |
| 起動時に動かす | 候補 `auto-launch` | Phase 0（0-10）で確かめてから決める |

- トレイは Slint の `SystemTrayIcon`、日付・時刻の選択は `DatePickerPopup` / `TimePickerPopup` を使う（別のクレートを足さない）。
- バージョンは `cargo add` で入れる（そのときの最新）。`Cargo.lock` はコミットする。
- **依存を増やすときは**、理由を PROGRESS.md の「決まったこと」に1行書く。ライセンスは MIT / Apache-2.0 / BSD / ISC / Zlib / Unicode 系 に限る。
- Rust は edition 2024。`rust-toolchain.toml` で stable を固定する（Slint の必要とする版以上）。

---

## コマンド

```sh
cargo run -p fuda-app                                   # 起動
cargo test --workspace                                  # テスト
cargo fmt --all                                         # 整形
cargo clippy --workspace --all-targets -- -D warnings   # 警告を0に
cargo deny check                                        # ライセンス・脆弱性（Phase 9 から必須）

# 画面を見ながら .slint を直す（ライブプレビュー）
SLINT_LIVE_PREVIEW=1 cargo run -p fuda-app --features dev-preview

# 動いているアプリを AI から調べる（Slint の MCP サーバー）
SLINT_EMIT_DEBUG_INFO=1 SLINT_MCP_PORT=9315 cargo run -p fuda-app --features dev-mcp

# 知らせを手で試すとき（時計をずらす。dev-clock のときだけ効く）
FUDA_FAKE_NOW="2026-11-19 09:29" cargo run -p fuda-app --features dev-clock
```

`crates/fuda-app/Cargo.toml` に次を書いておく:

```toml
[features]
dev-preview = ["slint/live-preview"]
dev-mcp = ["slint/mcp"]
dev-clock = []   # FUDA_FAKE_NOW を読む Clock を使う。リリースのビルドには入れない
```

Windows の PowerShell では環境変数を `$env:SLINT_MCP_PORT=9315; cargo run ...` のように書く。

---

## Slint 用の道具（最初に1回）

- **Slint 公式のスキル**（Claude Code の中で）:
  ```
  /plugin marketplace add slint-ui/ai-plugins
  /plugin install slint@slint
  ```
- **Slint の MCP サーバー**（動いているアプリの要素ツリー、アクセシビリティ情報、スクリーンショット、クリック・入力）:
  上の `dev-mcp` で起動してから、
  ```sh
  claude mcp add --transport http slint-app http://localhost:9315/mcp
  ```
- 機能名や環境変数は Slint の版で変わることがあります。Phase 0（ROADMAP 0-14）で動くことを確かめ、違っていたら **この節を直してから** 先へ進みます。

---

## ディレクトリ

```
Cargo.toml              # workspace
rust-toolchain.toml
data/holidays_jp.csv    # 祝日（内閣府の CSV を UTF-8 に直したもの。毎年更新）
tools/                  # 祝日の CSV を直すスクリプトなど
crates/
  fuda-core/            # ルールとデータ。Slint に依存しない。テストはここに厚く書く
  fuda-app/             # 画面とつなぎ。ビジネスルールを書かない
    ui/                 # .slint（app / notice / tray / theme / bridge / components / views）
    fonts/              # BIZ UDPゴシック と OFL.txt
docs/                   # SPEC / ROADMAP / PROGRESS / spike-report
spike/                  # Phase 0 の試しのコード（あとで消してよい）
```

細かい構成は SPEC §9.1。

---

## コードの決まり

### 全体
- 識別子は英語。コメント・ドキュメント・コミットメッセージは日本語でよい。
- 名前は SPEC §3 の「コード上の名前」に合わせる（`Card`, `Step`, `Place`, `Board`, `Column`, `CardColor`, `Reminder`, `Recurrence` …）。
- テスト以外で `unwrap` / `expect` / `panic!` を使わない（core は `#![deny(clippy::unwrap_used, clippy::expect_used)]`）。

### fuda-core
- **DB がただ一つの正しいデータ**。変更は必ず `service.rs` の関数を通す（SPEC §8.5 の不変条件を守るため）。
- 1つのユーザー操作 = 1つのトランザクション = 1つの `Command`（元に戻す の単位）。
- マイグレーションは足すだけ。**過去のファイルを書き換えない**。`PRAGMA user_version` で管理する。
- 時刻を直接取らない（`Local::now()` を書かない）。`Clock` を受け取る。
- カードの日付・時刻は **地域の時計どおりの文字列**（`'YYYY-MM-DD'`、`'HH:MM'`）で持つ。記録の時刻だけ UTC のミリ秒（SPEC §8.3）。
- 繰り返しの回は **保存しない**。見る期間のぶんだけ `schedule.rs` で計算する（SPEC §8.7）。
- 知らせを出すかどうかの判断は `reminder.rs` だけに書く（SPEC §8.8）。app 側で時刻を比べない。
- 不変条件ごとにテストを書く。DB は `Connection::open_in_memory()`。繰り返しと知らせは SPEC §9.7 のテストを必ず書く。

### fuda-app と .slint
- `.slint` には **表示のための状態だけ**。並べ方・日付の判定・繰り返し・知らせ・移動のルールは core に書く。
- Rust とのやりとりは `ui/bridge.slint` の `global Bridge` に集める。
- 色・余白・文字の大きさは `ui/theme.slint` の `Theme` からだけ取る。**数値や色を直接書かない**。知らせの窓も同じ `Theme` を使う。
- 文字の大きさは `Theme.font-base` からの倍数で決める（「最大」26px で崩れないように）。
- 画面の文字はすべて `@tr("...")`（日本語のまま）。
- 操作できる要素には必ず `accessible-role` と `accessible-label` を付ける。
- 変更のあとは「操作 → core → 今の画面のモデルを DB から作り直す」。部分更新の工夫は、遅いと測れたときだけ。
- パソコンの通知が失敗しても、知らせの窓は必ず出す（通知の失敗はログだけ）。

---

## UI を変えたときのチェック（全部「はい」になるまで完了にしない）

- [ ] この画面の目的は1つか。ボタンや設定を増やしたなら、本当に必要か。
- [ ] 言葉は用語集どおりか。アイコンだけのボタンはないか。
- [ ] 音・色だけで伝えていないか。
- [ ] 自動で消える表示、時間制限、勝手に動くものはないか。
- [ ] 「動きを減らす」がオンのとき、アニメーションは止まるか。
- [ ] 元に戻せるか。
- [ ] 文字の大きさ「最大」とウィンドウ最小サイズ（960×640）で、文字が切れたり重なったりしないか。
- [ ] キーボードだけで操作でき、フォーカスの枠が見えるか。ドラッグしなくてもできるか。
- [ ] 開き直して、同じ場所に戻るか。
- [ ] 日本語入力の確定の Enter で、意図しない操作が起きないか。
- [ ] （知らせにかかわるとき）ほかのアプリで入力中に知らせの窓が出ても、フォーカスを奪わないか。消すまで残るか。
- [ ] （MCP が使えるとき）アクセシビリティ情報で、操作できる要素すべてに名前があるか。

---

## コミットの前に必ず

1. `cargo fmt --all`
2. `cargo clippy --workspace --all-targets -- -D warnings` が警告なし
3. `cargo test --workspace` が全部通る
4. UI を変えたら、起動して目で確かめる。MCP が使えるなら、スクリーンショットとアクセシビリティ情報も確かめる。
5. 知らせを変えたら、`dev-clock` で時計をずらして、実際に知らせの窓が出ることを確かめる。
6. `docs/PROGRESS.md` を書き直す

---

## してはいけないこと

- SPEC にない画面・ボタン・設定を勝手に足す（案は PROGRESS.md の「まだ決まっていないこと・質問」に書く）
- 音・知らせ・アニメーションを既定でオンにする（知らせは、ユーザーが付けたものだけ出す）
- 音だけで知らせる、知らせの窓でフォーカスを奪う、知らせを時間で消す
- 確認ダイアログを増やす
- ネットワーク通信を入れる（利用状況の送信、更新の確認、祝日の取得も含む）
- 過去のマイグレーションの書き換え、DB ファイルを直接いじること
- テストを消したり `#[ignore]` にしたりして通すこと
- 大きな作り直しを、相談なしに始めること

---

## 報告と質問のしかた

- 報告は **結論 → 変えたこと（箇条書き3つまで）→ 次の一手** の順で、短く。
- 質問は **一度に1つ**。選択肢を出し、おすすめに印を付ける。
- 長い手順は番号付きで、1ステップ1行。
- 仕様にないことで迷ったら、作る前に聞く。小さなことは SPEC §2 の原則で決めて、PROGRESS.md の「決まったこと」に書く。
