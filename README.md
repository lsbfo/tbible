# Terminal Bible CLI

[![Build Status](https://github.com/clintaire/tbible/workflows/Daily%20Bible%20Verse%20Update/badge.svg)](https://github.com/clintaire/tbible/actions)
[![Crates.io](https://img.shields.io/crates/v/terminal-bible)](https://crates.io/crates/terminal-bible)
[![Downloads](https://img.shields.io/crates/d/terminal-bible)](https://crates.io/crates/terminal-bible)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A lightweight command-line tool for reading Bible verses directly in your terminal.

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

## Install

```bash
git clone https://github.com/clintaire/tbible.git
cd tbible
./build.sh
```

## Usage

```bash
./bible                    # Daily verse
./bible john3-16          # John 3:16
./bible psalm23:1         # Psalm 23:1
./bible mt5:3             # Matthew 5:3
./bible gen ch 1vs1-3     # Genesis 1:1-3
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

```bash
./bible john3-16
./bible psalm23:1
./bible romans8:28
./bible phil4:13
./bible jer29:11
./bible psalm ch 23
./bible john ch 3vs1-21
./bible mt ch 5vs3-12
```
</details>

<details>
<summary><strong>Troubleshooting</strong></summary>

```bash
rustup update stable
cargo clean && ./build.sh
chmod +x build.sh bible
curl https://bible-api.com/john+3:16
```
</details>

## API

[![API Status](https://img.shields.io/website?url=https://bible-api.com&label=API%20Status&color=green)](https://bible-api.com)

Uses [bible-api.com](https://bible-api.com) for World English Bible (Public Domain).

## License

MIT License - see [LICENSE](LICENSE) file.
