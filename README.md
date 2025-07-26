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
## Today's Verse (2025-07-26)


    ╭─────────────────────────────────────────────────────────╮
    │                    TERMINAL BIBLE                       │
    ╰─────────────────────────────────────────────────────────╯

    ❝ PHILIPPIANS 4 13                                     NKJV
    ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

      1: I can do all things through Christ, who
         strengthens me.

    ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
<!-- DAILY_VERSE_END -->


## Quick Start

```bash
git clone https://github.com/clintaire/terminal-bible.git
cd terminal-bible
./build.sh
./bible
./bible john3-16
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

### Method 4: Homebrew (macOS/Linux)
```bash
# After publishing to Homebrew (future)
brew install terminal-bible
terminal-bible john3-16

# Or install from source via Homebrew
brew install --HEAD https://github.com/clintaire/terminal-bible.git
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

## Testing

### Local Testing (Recommended)
```bash
# Create scripts directory and test script
mkdir -p scripts
cat > scripts/test.sh << 'EOF'
#!/bin/bash
set -e
echo "=== Testing Terminal Bible Workflow ==="
cargo build --release
mkdir -p data/
./target/release/terminal-bible > data/daily-verse-$(date +%Y-%m-%d).txt 2>&1
echo "Raw output:"
cat data/daily-verse-$(date +%Y-%m-%d).txt
VERSE_CONTENT=$(cat data/daily-verse-$(date +%Y-%m-%d).txt | \
  sed 's/\x1b\[[0-9;]*m//g' | \
  sed 's/[╭╮╰╯─━❝✳]//g' | \
  sed 's/^[ ]*//g' | \
  grep -v "^TERMINAL BIBLE$" | \
  grep -v "^$" | \
  head -20)
echo "Cleaned content:"
echo "$VERSE_CONTENT"
echo "=== ✅ Test completed successfully! ==="
EOF

chmod +x scripts/test.sh
./scripts/test.sh
```

### Manual Component Testing
```bash
# Test CLI directly
cargo build --release
./target/release/terminal-bible john3-16

# Test different formats
./target/release/terminal-bible psalm ch 23
./target/release/terminal-bible mt5:3
```

### GitHub Actions Testing
```bash
# Push changes and monitor in GitHub Actions
git add .
git commit -m "Test changes"
git push

# Then go to: GitHub → Actions → "Daily Bible Verse Update"
# Click "Run workflow" to test manually
```

### API Testing
```bash
# Test bible-api.com directly
curl "https://bible-api.com/john+3:16"

# Test with different verses
curl "https://bible-api.com/psalm+23:1"

# Check API status
curl -I "https://bible-api.com"
```

## Project Structure

```
terminal-bible/
├── Cargo.toml              # Dependencies and config
├── src/main.rs             # Main application code
├── build.sh                # Build script
├── bible                   # Compiled binary
├── .github/workflows/      # Automation scripts
├── scripts/                # Utility scripts
│   └── test.sh             # Local testing script
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

## Troubleshooting & Debugging

### Build Issues
```bash
# Update Rust toolchain
rustup update stable
rustup default stable

# Clean everything and rebuild
cargo clean
rm -f ./bible
./build.sh

# Check Rust version
rustc --version
cargo --version

# Verbose build for debugging
cargo build --release --verbose
```

### Runtime Issues
```bash
# Check if binary exists and is executable
ls -la ./bible
file ./bible

# Make sure it's executable
chmod +x ./bible

# Test with debug output
RUST_LOG=debug ./bible john3-16

# Test with different book names
./bible --help          # Should show error (no help implemented)
./bible invalid-book    # See error handling
```

### Network/API Issues
```bash
# Test internet connectivity
ping google.com

# Test bible-api.com specifically
curl -v "https://bible-api.com/john+3:16"

# Test with different verses
curl "https://bible-api.com/psalm+91:1"
curl "https://bible-api.com/romans+8:28"

# Check DNS resolution
nslookup bible-api.com

# Test with proxy (if behind corporate firewall)
curl --proxy http://proxy:port "https://bible-api.com/john+3:16"
```

### GitHub Actions Debugging
```bash
# Check GitHub Actions in the web interface
# Go to: Repository → Actions → Workflow runs
# Click on a specific run to see detailed logs

# Check workflow syntax locally
grep -n "name:" .github/workflows/*.yml

# Validate YAML syntax
python3 -c "import yaml; yaml.safe_load(open('.github/workflows/daily-bible-update.yml'))"
```

### Permission Issues
```bash
# Linux/macOS: Fix file permissions
chmod +x build.sh
chmod +x bible

# Windows: Run in PowerShell as administrator
# Or use Git Bash with admin privileges

# WSL: Check file system type
mount | grep $(pwd)
```

### Performance Debugging
```bash
# Time the execution
time ./bible john3-16

# Check memory usage
/usr/bin/time -v ./bible psalm23:1  # Linux
/usr/bin/time -l ./bible psalm23:1  # macOS

# Profile network requests
strace -e network ./bible john3-16  # Linux only

# Check what files are accessed
strace -e file ./bible john3-16     # Linux only
```

### Common Error Solutions

**"command not found"**
```bash
# Make sure you're in the right directory
pwd
ls -la bible

# Check PATH (for global install)
echo $PATH
which terminal-bible
```

**"Permission denied"**
```bash
chmod +x ./bible
chmod +x build.sh
```

**"Connection refused" or "Network error"**
```bash
# Check internet connection
ping 8.8.8.8

# Check if behind firewall
curl -I https://bible-api.com

# Try different DNS
echo "nameserver 8.8.8.8" | sudo tee /etc/resolv.conf
```

**"Verse not found"**
```bash
# Check book name spelling
./bible list  # (if implemented)

# Try different abbreviations
./bible psalm23:1    # instead of ps23:1
./bible john3-16     # instead of jn3-16
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
./bible romans8:28 | fold -w 40       # Wrap text at 40 chars
```

### Create Reading Plans
```bash
# Daily reading file
./bible psalm ch 1 > daily_reading.txt
./bible psalm ch 2 >> daily_reading.txt

# Weekly plan
for i in {1..7}; do ./bible psalm ch $i >> weekly_plan.txt; done
```

### Integration with Other Tools
```bash
# Use with cron for daily notifications
echo "0 8 * * * cd /path/to/terminal-bible && ./bible | mail -s 'Daily Verse' user@example.com" | crontab -

# Use with tmux status bar
tmux set-option -g status-right "#(cd /path/to/terminal-bible && ./bible | head -1)"

# Use with conky
conky -t "$(cd /path/to/terminal-bible && ./bible)"
```

## License

MIT License - feel free to use, modify, and distribute.

## Contributing

1. Fork the repository
2. Make your changes (the code is straightforward)
3. Test with `./build.sh && ./bible john3-16`
4. Run `./scripts/test.sh` to verify everything works
5. Submit a pull request

## Support

- **Issues:** Use GitHub Issues
- **Feature requests:** Open an issue with your idea  
- **API problems:** Check [bible-api.com](https://bible-api.com) status
- **Testing:** Use `./scripts/test.sh` to verify functionality locally

---

**Built for speed, designed for daily use.**
