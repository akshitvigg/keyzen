#!/bin/bash

echo "Installing keyzen typing test..."

echo "Building keyzen..."
cargo build --release

if [ -w /usr/local/bin ]; then
    sudo ln -sf "$(pwd)/target/release/keyzen" /usr/local/bin/keyzen
    echo "✅ keyzen installed to /usr/local/bin/keyzen"
elif [ -w ~/.local/bin ]; then
    mkdir -p ~/.local/bin
    ln -sf "$(pwd)/target/release/keyzen" ~/.local/bin/keyzen
    echo "✅ keyzen installed to ~/.local/bin/keyzen"
    echo "💡 Add ~/.local/bin to your PATH if not already added"
else
    echo "⚠️  Could not install to system directories"
    echo "💡 You can run keyzen from: $(pwd)/target/release/keyzen"
fi

echo ""
echo "🎉 Installation complete!"
echo ""
echo "Usage examples:"
echo "  keyzen start                    # 30s English test"
echo "  keyzen start -d 60              # 60s English test"
echo "  keyzen start -l rust            # 30s Rust test"
echo "  keyzen start -d 45 -l javascript # 45s JavaScript test"
echo "  keyzen start --list-langs       # Show all languages"
echo "  keyzen --help                   # Show help"
