# Terminal Bible CLI

[![Build Status](https://github.com/clintaire/tbible/workflows/Daily%20Bible%20Verse%20Update/badge.svg)](https://github.com/clintaire/tbible/actions)
[![Crates.io](https://img.shields.io/crates/v/terminal-bible)](https://crates.io/crates/terminal-bible)
[![Downloads](https://img.shields.io/crates/d/terminal-bible)](https://crates.io/crates/terminal-bible)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A lightweight command-line tool for reading Bible verses directly in your terminal with daily verse rotation and flexible input parsing.

## Contents

| Section | Description |
|---------|-------------|
| [Daily Verse](#daily-verse) | Today's automated Bible verse |
| [Quick Start](#quick-start) | Get up and running fast |
| [Usage](#usage) | Basic command examples |
| [Installation](#installation) | Build and install methods |
| [Book Abbreviations](#book-abbreviations) | Supported Bible book shortcuts |
| [Examples](#examples) | Common usage patterns |
| [Troubleshooting](#troubleshooting) | Fix common issues |
| [Development](#development) | Contributing guide |
| [License](#license) | MIT License |

<!-- DAILY_VERSE_START -->
## Daily Verse

    ╭─────────────────────────────────────────────────────────╮
    │                    TERMINAL BIBLE                       │
    ╰─────────────────────────────────────────────────────────╯

    ❝ ROMANS 8 28                                          NKJV
    ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

      1: We know that all things work together for good
         for those who love God, to those who are called
         according to his purpose.

    ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
<!-- DAILY_VERSE_END -->

## Quick Start

```bash
git clone https://github.com/clintaire/tbible.git
cd tbible
./build.sh
./bible john3-16
```

## Usage

```bash
./bible
./bible john3-16
./bible psalm23:1
./bible mt5:3
./bible gen ch 1vs1-3
```

## Installation

**Build Script (Recommended):**
```bash
git clone https://github.com/clintaire/tbible.git
cd tbible
chmod +x build.sh
./build.sh
```

**Manual Build:**
```bash
cargo build --release
cp ./target/release/terminal-bible ./bible
```

**Global Install:**
```bash
cargo install --path .
```

## Book Abbreviations

| Old Testament | New Testament |
|---------------|---------------|
| gen, ex, lev, num, deut, josh, judg, ruth | mt, mk, lk, jn, acts, rom, 1cor, 2cor |
| 1sam, 2sam, 1ki, 2ki, 1chr, 2chr, ezra, neh | gal, eph, phil, col, 1th, 2th, 1tim, 2tim |
| est, job, ps/psalm, prov, eccl, song, isa | tit, phlm, heb, jas, 1pet, 2pet, 1jn, 2jn |
| jer, lam, ezek, dan, hos, joel, amos, oba | 3jn, jude, rev |
| jon, mic, nah, hab, zep, hag, zech, mal | |

<details>
<summary><strong>Examples</strong></summary>

**Popular Verses:**
```bash
./bible john3-16
./bible psalm23:1
./bible romans8:28
./bible phil4:13
./bible jer29:11
```

**Chapters:**
```bash
./bible psalm ch 23
./bible john ch 3vs1-21
./bible mt ch 5vs3-12
```

**Input Formats:**
```bash
john3-16
john 3:16
john ch 3 vs 16
psalm ch 23vs1-6
mt5:3
```

</details>

<details>
<summary><strong>Troubleshooting</strong></summary>

**Build Issues:**
```bash
rustup update stable
cargo clean && ./build.sh
```

**Permission Issues:**
```bash
chmod +x build.sh bible
```

**API Issues:**
```bash
curl https://bible-api.com/john+3:16
ping google.com
```

**Common Errors:**
- "command not found" → Check if in correct directory
- "Verse not found" → Check book name spelling

</details>

<details>
<summary><strong>Development</strong></summary>

**Quick Development Cycle:**
```bash
# Edit src/main.rs
./build.sh
./bible john3-16
```

**Key Files:**
- `src/main.rs` - Main application (under 200 lines)
- `build.sh` - Build script
- `Cargo.toml` - Dependencies

**Adding Features:**
- Edit `parse_reference()` function for new syntax
- Edit `expand_book_name()` for new abbreviations

</details>

## License

MIT License - see [LICENSE](LICENSE) file.

## API

[![API Status](https://img.shields.io/website?url=https://bible-api.com&label=API%20Status&color=green)](https://bible-api.com)

This project uses the [bible-api.com](https://bible-api.com) service to fetch Bible verses. 
The API provides free access to the World English Bible (Public Domain) translation.

---

**Built for speed, designed for daily use.**
