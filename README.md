# Terminal Bible CLI

A minimal, fast terminal Bible reader with intuitive commands and daily verses.

## Features

- Ultra-fast startup with minimal dependencies
- Short, memorable commands (`./bible john3-16`)
- Daily verse rotation through popular scriptures  
- Smart parsing for multiple input formats
- Complete Bible support with book abbreviations
- API-powered for always fresh content
- Automated daily updates via GitHub Actions

<!-- DAILY_VERSE_START -->
## Today's Verse

*This section will be automatically updated by GitHub Actions*
<!-- DAILY_VERSE_END -->

## Quick Start

```bash
git clone https://github.com/clintaire/terminal-bible.git
cd terminal-bible
./build.sh
./bible
./bible john3-16
```

## Commands

The CLI is designed for fast, natural typing with flexible syntax:

### Daily Verse
```bash
./bible                     # Get today's verse
```

### Simple Formats
```bash
./bible john3-16            # John 3:16
./bible psalm23:1           # Psalm 23:1
./bible gen1:1              # Genesis 1:1
./bible mt5:3               # Matthew 5:3
./bible rev22:20            # Revelation 22:20
```

### Natural Language Style
```bash
./bible psalm ch 3vs4-18    # Psalm 3:4-18
./bible john ch 3 vs 16     # John 3:16
./bible romans ch 8vs28     # Romans 8:28
./bible genesis ch 1vs1-3   # Genesis 1:1-3
```

### Flexible Input
```bash
./bible john 3:16           # Spaces work
./bible john3-16            # Dashes work  
./bible john3:16            # Colons work
./bible john ch3 vs16       # Natural language
```

## Book Abbreviations

**Old Testament:**
```
gen, ex, lev, num, deut, josh, judg, ruth, 1sam, 2sam, 
1ki, 2ki, 1chr, 2chr, ezra, neh, est, job, ps/psalm, 
prov, eccl, song, isa, jer, lam, ezek, dan, hos, joel, 
amos, oba, jon, mic, nah, hab, zep, hag, zech, mal
```

**New Testament:**
```
mt, mk, lk, jn, acts, rom, 1cor, 2cor, gal, eph, phil, 
col, 1th, 2th, 1tim, 2tim, tit, phlm, heb, jas, 1pet, 
2pet, 1jn, 2jn, 3jn, jude, rev
```

## Installation

### Method 1: Build Script (Recommended)
```bash
git clone https://github.com/clintaire/terminal-bible.git
cd terminal-bible
chmod +x build.sh
./build.sh
./bible john3-16
```

### Method 2: Manual Build
```bash
cargo build --release
cp ./target/release/terminal-bible ./bible
./bible
```

### Method 3: Global Install
```bash
cargo install --path .
# Creates ~/.cargo/bin/terminal-bible (add ~/.cargo/bin to PATH)
terminal-bible john3-16     # Works from anywhere
```

## Usage Examples

### Daily Reading
```bash
./bible                     # Get today's verse
```

### Popular Verses
```bash
./bible john3-16            # John 3:16
./bible psalm23:1           # The Lord is my shepherd
./bible romans8:28          # All things work together
./bible phil4:13            # I can do all things
./bible jer29:11            # Plans to prosper you
./bible prov3:5             # Trust in the Lord
```

### Chapters and Ranges
```bash
./bible psalm ch 23         # Entire Psalm 23
./bible john ch 3vs1-21     # John 3:1-21
./bible mt ch 5vs3-12       # Matthew 5:3-12 (Beatitudes)
./bible gen ch 1vs1-10      # Genesis 1:1-10
```

### Multiple Input Styles
```bash
# All of these work for the same verse:
./bible john 3:16
./bible john3-16  
./bible john ch 3 vs 16
./bible jn3:16
./bible john3vs16
```

## Project Structure

```
terminal-bible/
├── Cargo.toml              # Dependencies and config
├── src/main.rs             # Main application code
├── build.sh                # Build script
├── bible                   # Compiled binary
├── .github/workflows/      # Automation scripts
└── README.md               # Documentation
```

## Automation

The repository automatically:
- Updates daily verse at 6 AM UTC
- Tests API endpoints for reliability  
- Updates README with today's verse
- Creates monthly archives of verses
- Builds cross-platform releases

## Technical Details

**API Source:** [bible-api.com](https://bible-api.com) (reliable, free)  
**Translation:** World English Bible (Public Domain)  
**Storage:** No local cache needed - always fresh content  

**Performance:**
- Build time: ~1-2 seconds
- Startup time: ~50ms
- API response: ~100-200ms
- Memory usage: ~2MB
- Binary size: ~3MB

## Development

### Quick Development Cycle
```bash
# Edit src/main.rs
./build.sh                  # Rebuilds and copies to ./bible
./bible john3-16            # Test immediately
```

### Adding Features
- Edit `parse_reference()` function for new syntax
- Edit `expand_book_name()` for new abbreviations
- The code is under 200 lines - easy to modify

### Testing Different Formats
```bash
./bible john3-16
./bible psalm ch 23vs1-6  
./bible mt5:3
./bible genesis ch 1vs1
```

## Design Philosophy

**Muscle Memory First** - Commands should be fast to type and easy to remember

**Natural Language** - `psalm ch 3vs4` feels more natural than complex flags

**Minimal Dependencies** - Only essential dependencies for speed and reliability

**Zero Configuration** - Works immediately after build

**Smart Parsing** - Handles input variations naturally

## Troubleshooting

### Build Issues
```bash
rustup update               # Update Rust
cargo clean                 # Clean build cache
./build.sh                  # Rebuild
```

### Network Issues
```bash
curl "https://bible-api.com/john+3:16"  # Test API manually
ping google.com                         # Check internet connection
```

### Command Issues
```bash
ls -la ./bible              # Check if binary exists
chmod +x ./bible            # Make executable
./bible                     # Test with simple command
```

## Advanced Usage

### Batch Reading
```bash
./bible john3-16 && ./bible psalm23:1 && ./bible romans8:28
```

### Pipe to Other Tools
```bash
./bible psalm23:1 | grep "shepherd"
./bible john3-16 | wc -w              # Word count
```

### Create Reading Plans
```bash
./bible psalm ch 1 > daily_reading.txt
./bible psalm ch 2 >> daily_reading.txt
```

## License

MIT License - feel free to use, modify, and distribute.

## Contributing

1. Fork the repository
2. Make your changes (the code is straightforward)
3. Test with `./build.sh && ./bible john3-16`
4. Submit a pull request

## Support

- **Issues:** Use GitHub Issues
- **Feature requests:** Open an issue with your idea  
- **API problems:** Check [bible-api.com](https://bible-api.com) status

---

**Built for speed, designed for daily use.**
