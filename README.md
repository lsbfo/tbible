[![Build Status](https://github.com/clintaire/tbible/workflows/Daily%20Bible%20Verse%20Update/badge.svg)](https://github.com/clintaire/tbible/actions)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

![Terminal Bible CLI](https://camo.githubusercontent.com/2722992d519a722218f896d5f5231d49f337aaff4514e78bd59ac935334e916a/68747470733a2f2f692e696d6775722e636f6d2f77617856496d762e706e67)

# Terminal Bible CLI

A lightweight command-line tool for reading Bible verses directly in your terminal.

[API](#api) — [Install](#install) — [License](#license) — [Usage](#usage)

<!-- DAILY_VERSE_START -->
## Daily Verse


    ╭─────────────────────────────────────────────────────────╮
    │                    TERMINAL BIBLE                       │
    ╰─────────────────────────────────────────────────────────╯

    ❝ PSALM 46 1                                           NKJV
    ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

      1: God is our refuge and strength, a very present
         help in trouble.

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
./bible
./bible john3-16
./bible psalm23:1
./bible mt5:3
./bible gen ch 1vs1-3
```

<details>
<summary><strong>Book Abbreviations</strong></summary>

Use these short forms instead of full book names:

**Old Testament:**
```
gen, ex, lev, num, deut, josh, judg, ruth
1sam, 2sam, 1ki, 2ki, 1chr, 2chr, ezra, neh, est, job
ps/psalm, prov, eccl, song, isa, jer, lam, ezek, dan
hos, joel, amos, oba, jon, mic, nah, hab, zep, hag, zech, mal
```

**New Testament:**
```
mt, mk, lk, jn, acts, rom, 1cor, 2cor, gal, eph, phil, col
1th, 2th, 1tim, 2tim, tit, phlm, heb, jas, 1pet, 2pet
1jn, 2jn, 3jn, jude, rev
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

This project fetches Bible verses from [bible-api.com](https://bible-api.com), a free service providing the World English Bible translation. The API is reliable, fast, and requires no authentication.

## License

[MIT](./LICENSE) &copy; [Clint Airé](https://github.com/clintaire)
