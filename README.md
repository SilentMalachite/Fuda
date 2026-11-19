# fuda

English | [日本語](README.ja.md)

**fuda** is an offline-first desktop application for managing to-dos, schedules, notes, and Kanban boards without cognitive fatigue—designed with accessibility and comfort in mind for people with ASD, ADHD, hearing loss, and sensory sensitivities.

> **Status:** Early development (**Phase 0: Technical Verification**). The production crates (`fuda-core` and `fuda-app`) are not created yet; current experiments run in [`spike/fuda-spike`](spike/fuda-spike).

---

## Overview

- **One unified item ("Card"):** Every item in fuda is a single type of **Card** that lives in exactly one of five places:
  1. **Inbox** (*Toriaezu* — quick capture)
  2. **Tasks** (*Yarukoto* — checkable items)
  3. **Events** (*Yotei* — scheduled items and planner view)
  4. **Notes** (*Memo* — open-ended notes)
  5. **Boards** (*Bōdo* — column-based Kanban workflow)
- **Schedule view across places:** The Events view displays a daily, weekly, or monthly planner showing dated cards across Tasks and Boards without moving them out of their original place.
- **Visual-first reminders (no sound required):** Instead of relying on audio alerts, fuda brings up a dedicated **Notice Window**—always on top, non-focus-stealing so it never interrupts typing, and persistent until you dismiss it.
- **100% local & offline:** No cloud accounts, no telemetry, and no network requests at runtime.

---

## Core UX Principles

From [docs/SPEC.md](docs/SPEC.md) §2:

1. **One purpose per screen:** Each view focuses on one clear goal with a single primary action and an explicit heading.
2. **Do not make the user remember:** Everything auto-saves (no Save button). Reopening fuda restores the exact previous screen, card, and window size.
3. **Do not rely on sound:** Audio is off by default. Reminders appear visually in the Notice Window.
4. **Never move things unexpectedly:** No automatic list reordering, layout shifts, or auto-scrolling without user action. Animations are off by default (and ≤ 150ms when enabled). The Notice Window never steals keyboard focus.
5. **Do not rush with timers:** No auto-dismissing toast messages, time limits, or second-by-second countdowns.
6. **Never blame the user:** Overdue items are not highlighted in red or warning badges. No streaks, points, or incomplete-count badges.
7. **Always undoable:** Deleting moves cards to Trash. Data-changing actions support Undo (`Ctrl+Z` / `⌘+Z`). Only "Empty Trash" shows a confirmation dialog.
8. **Low stimulation & accessible:** No transparency, blur, or flashing. Meaning is never conveyed by color alone (weekends and holidays are shown with text labels; card colors include text names).

---

## Development Status

fuda is currently in **Phase 0 (Technical Verification)** before building the core and application crates (`crates/fuda-core` and `crates/fuda-app` start in Phase 1).

Verified so far on macOS in [`spike/fuda-spike`](spike/fuda-spike):

- **0-1a:** Workspace setup with `slint` 1.19.0 and `rusqlite` (bundled SQLite 3.54.1)
- **0-2:** Bundled **BIZ UDPGothic** font rendering, Japanese punctuation/symbols, and fixed-width time column alignment
- **0-3a:** `LineEdit` and `TextEdit` Japanese IME preedit, multiline newlines, and ensuring IME confirmation `Enter` does not trigger `accepted`
- **0-4:** Dynamic `Theme.font-base` scaling (16px → 26px) across text/spacing and line spacing via `line-height-factor: 1.6`

Next planned technical checks include drag-and-drop (`0-5`), date/time pickers (`0-6`), non-focus-stealing always-on-top Notice Window (`0-7`), system tray background execution (`0-8`), and Windows verification (`0-1b`, `0-3b`, etc.).

### Project Documentation (Japanese)

- Specification: [docs/SPEC.md](docs/SPEC.md)
- Roadmap: [docs/ROADMAP.md](docs/ROADMAP.md)
- Progress Log: [docs/PROGRESS.md](docs/PROGRESS.md)

---

## Tech Stack

- **Language:** Rust (edition 2024)
- **UI Toolkit:** [Slint](https://slint.dev/) `1.19.0`
- **Database:** SQLite via `rusqlite` (`bundled`, WAL mode)
- **Bundled Font:** BIZ UDPGothic (`Regular` and `Bold`, SIL OFL 1.1)
- **Japanese Public Holidays (planned for Phase 3):** Offline CSV derived from the Cabinet Office of Japan (*Naikakufu*) public holiday dataset

---

## Running the Spike

Ensure Rust (`stable` toolchain) is installed, then run from the repository root:

```sh
# Launch the Phase 0 technical verification window
cargo run -p fuda-spike

# Run workspace tests
cargo test --workspace

# Format and lint
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
```

---

## License

- **fuda:** Licensed under the **GNU General Public License v3.0 only (`GPL-3.0-only`)**. See [LICENSE](LICENSE).
- **Slint:** Used under the **GNU GPLv3 (`GPL-3.0-only`)** license option.
- **BIZ UDPGothic:** Licensed under the **SIL Open Font License 1.1**. See [spike/fuda-spike/fonts/OFL.txt](spike/fuda-spike/fonts/OFL.txt).
- **Japanese Public Holiday Data (planned for Phase 3):** Will use a processed offline copy of the Cabinet Office of Japan's [National Holidays CSV](https://www8.cao.go.jp/chosei/shukujitsu/gaiyou.html) in accordance with its terms of use.
