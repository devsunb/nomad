#!/bin/bash
set -e

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${BLUE}🚀 Building Nomad Release Artifacts${NC}"

# Configuration
TAG="nightly"
REPO="devsunb/nomad"
ARTIFACTS_DIR="./artifacts"

# Detect architecture
ARCH=$(uname -m)
if [ "$ARCH" = "arm64" ]; then
  RUST_ARCH="aarch64"
elif [ "$ARCH" = "x86_64" ]; then
  RUST_ARCH="x86_64"
else
  echo -e "${RED}❌ Unsupported architecture: $ARCH${NC}"
  exit 1
fi

# Detect OS
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
if [ "$OS" = "darwin" ]; then
  OS_NAME="macos"
elif [ "$OS" = "linux" ]; then
  OS_NAME="linux"
else
  echo -e "${RED}❌ Unsupported OS: $OS${NC}"
  exit 1
fi

echo -e "${GREEN}✓ Detected platform: $OS_NAME-$RUST_ARCH${NC}"

# Clean artifacts directory
rm -rf "$ARTIFACTS_DIR"
mkdir -p "$ARTIFACTS_DIR"

# Function to build for a specific Neovim version
build_for_neovim_version() {
  local NVIM_VERSION=$1
  local NIGHTLY_FLAG=$2
  local BUILD_DIR="$ARTIFACTS_DIR/build-nvim-${NVIM_VERSION}"

  echo -e "${BLUE}📦 Building for Neovim ${NVIM_VERSION}...${NC}"

  # Clean previous build
  cargo clean

  # Build the Rust binary
  if [ "$NIGHTLY_FLAG" = "true" ]; then
    echo -e "${GREEN}  Building with nightly features...${NC}"
    cargo build --release --package nomad-neovim --features neovim-nightly
  else
    echo -e "${GREEN}  Building for stable...${NC}"
    cargo build --release --package nomad-neovim
  fi

  # Create temporary build directory
  mkdir -p "$BUILD_DIR/lua"

  # Copy the built library
  if [ "$OS_NAME" = "macos" ]; then
    LIB_NAME="libnomad.dylib"
    TARGET_LIB="nomad.so"
  else
    LIB_NAME="libnomad.so"
    TARGET_LIB="nomad.so"
  fi

  cp "target/release/$LIB_NAME" "$BUILD_DIR/lua/$TARGET_LIB"

  # Copy Lua files
  cp -r lua/nomad "$BUILD_DIR/lua/"

  # Create archive name
  ARCHIVE_NAME="nomad-${TAG}-for-neovim-${NVIM_VERSION}-${OS_NAME}-${RUST_ARCH}.tar.gz"

  # Create tarball
  echo -e "${GREEN}  Creating archive: ${ARCHIVE_NAME}${NC}"
  tar -czf "$ARTIFACTS_DIR/$ARCHIVE_NAME" -C "$BUILD_DIR" lua

  # Clean build directory
  rm -rf "$BUILD_DIR"

  echo -e "${GREEN}✓ Built ${ARCHIVE_NAME}${NC}"
}

# Build for Neovim 0.11 (stable)
build_for_neovim_version "0.11" "false"

# Build for Neovim 0.12 (nightly)
build_for_neovim_version "0.12-nightly" "true"

echo -e "${BLUE}📋 Built artifacts:${NC}"
ls -lh "$ARTIFACTS_DIR"/*.tar.gz

# Upload to GitHub
echo -e "${BLUE}🚀 Uploading to GitHub Release...${NC}"

# Check if gh CLI is installed
if ! command -v gh &>/dev/null; then
  echo -e "${RED}❌ GitHub CLI (gh) is not installed.${NC}"
  echo -e "${BLUE}Please install it: brew install gh${NC}"
  echo -e "${BLUE}Then run: gh auth login${NC}"
  exit 1
fi

# Delete existing release and tag
echo -e "${BLUE}🗑️  Deleting old release and tag...${NC}"
gh release delete "$TAG" --repo "$REPO" --yes 2>/dev/null || echo "  No existing release to delete"
git push --delete origin "$TAG" 2>/dev/null || echo "  No existing tag to delete"

# Create new tag
echo -e "${BLUE}🏷️  Creating new tag...${NC}"
git tag -f "$TAG"
git push -f origin "$TAG"

# Create new release with artifacts
echo -e "${BLUE}📤 Creating release and uploading artifacts...${NC}"
gh release create "$TAG" \
  -R "$REPO" \
  -t "$TAG" \
  -n "" \
  "$ARTIFACTS_DIR"/*.tar.gz

echo -e "${GREEN}✅ Release complete!${NC}"
echo -e "${BLUE}View at: https://github.com/$REPO/releases/tag/$TAG${NC}"
