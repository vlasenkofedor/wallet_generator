#!/bin/bash

set -e  # Exit on error

echo "🔨 Building TRON Wallet Generator..."
echo ""

# Clean previous builds
echo "📦 Cleaning previous builds..."
cargo clean

# Build with optimizations
echo "🚀 Compiling with release optimizations..."
cargo build --release

# Get binary path
BINARY="target/release/wallet_generator"

if [ ! -f "$BINARY" ]; then
    echo "❌ Error: Binary not found at $BINARY"
    exit 1
fi

# Show original size
ORIGINAL_SIZE=$(du -h "$BINARY" | cut -f1)
echo "📊 Original binary size: $ORIGINAL_SIZE"

# Strip debug symbols
echo "✂️  Stripping debug symbols..."
strip "$BINARY"

STRIPPED_SIZE=$(du -h "$BINARY" | cut -f1)
echo "📊 Stripped binary size: $STRIPPED_SIZE"

# Compress with UPX (if available and not on macOS)
if command -v upx &> /dev/null; then
    echo "🗜️  Compressing with UPX..."
    
    # Try with --force-macos on macOS
    if [[ "$OSTYPE" == "darwin"* ]]; then
        echo "   (macOS detected - using --force-macos)"
        upx --best --force-macos "$BINARY" 2>/dev/null && {
            COMPRESSED_SIZE=$(du -h "$BINARY" | cut -f1)
            echo "📊 Compressed binary size: $COMPRESSED_SIZE"
        } || {
            echo "⚠️  UPX compression not supported on this macOS version"
            echo "   Binary is stripped but not compressed"
        }
    else
        upx --best --lzma "$BINARY" 2>/dev/null || upx --best "$BINARY"
        COMPRESSED_SIZE=$(du -h "$BINARY" | cut -f1)
        echo "📊 Compressed binary size: $COMPRESSED_SIZE"
    fi
else
    echo "⚠️  UPX not found. Skipping compression."
    echo "   Install with: brew install upx"
fi

echo ""
echo "✅ Build complete!"
echo "📍 Binary location: $BINARY"
echo ""
echo "Usage:"
echo "  CPU version: $BINARY <suffix>"
echo "  GPU version: $BINARY <suffix> --gpu"
echo ""
echo "Examples:"
echo "  $BINARY 888888"
echo "  $BINARY TRON --gpu"
echo ""
echo "To install system-wide:"
echo "  sudo cp $BINARY /usr/local/bin/"