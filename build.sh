#!/bin/bash
# Interactive build menu - delegates to Makefile (Single Source of Truth)

# Colors
BLUE='\033[0;34m'
GREEN='\033[0;32m'
NC='\033[0m'

echo -e "${BLUE}🚀 yCard Build System${NC}"
echo "===================="
echo ""
echo "What would you like to build?"
echo ""
echo "1) Everything (recommended)"
echo "2) Development build (debug, faster)"
echo "3) Full ecosystem (includes TypeScript/LSP)"
echo "4) Just install CLI globally"
echo "5) Show all options"
echo ""

read -p "Choice (1-5): " -n 1 -r
echo ""
echo ""

case $REPLY in
    1)
        echo "Building everything..."
        make all
        echo ""
        read -p "Install CLI globally? (y/N): " -n 1 -r
        echo ""
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            make install
        fi
        ;;
    2)
        echo "Development build..."
        make dev
        ;;
    3)
        echo "Full ecosystem build..."
        make full
        ;;
    4)
        echo "Installing CLI..."
        make install
        ;;
    5)
        echo "All available options:"
        make help
        ;;
    *)
        echo "Building default (everything)..."
        make all
        ;;
esac

echo ""
echo -e "${GREEN}✅ Done! Use 'make help' to see all build options.${NC}"