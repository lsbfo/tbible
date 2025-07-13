#!/bin/bash
set -e

echo "=== Testing Terminal Bible Workflow ==="

# Build
echo "Building..."
cargo build --release

# Create data directory
mkdir -p data/

# Fetch verse
echo "Fetching daily verse..."
./target/release/terminal-bible > data/daily-verse-$(date +%Y-%m-%d).txt 2>&1

echo "Raw output:"
cat data/daily-verse-$(date +%Y-%m-%d).txt

# Clean content
echo "Cleaning content..."
VERSE_CONTENT=$(cat data/daily-verse-$(date +%Y-%m-%d).txt | \
  sed 's/\x1b\[[0-9;]*m//g' | \
  sed 's/[╭╮╰╯─━❝✳]//g' | \
  sed 's/^[ ]*//g' | \
  grep -v "^TERMINAL BIBLE$" | \
  grep -v "^$" | \
  head -20)

echo "Cleaned content:"
echo "$VERSE_CONTENT"

# Test README update (backup first)
cp README.md README.md.backup

# Create temp section
cat > temp_readme_section << EOF
<!-- DAILY_VERSE_START -->
## Today's Verse ($(date +%Y-%m-%d))

\`\`\`
$VERSE_CONTENT
\`\`\`

*Updated automatically via GitHub Actions*
<!-- DAILY_VERSE_END -->
