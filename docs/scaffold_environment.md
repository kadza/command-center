# Point 1: Project Scaffolding & Environment

This document captures the step-by-step instructions to bootstrap the repository structure and verify both local and Raspberry Pi environments.

## 1. Create the top-level folders
```bash
mkdir robot-pi robot-web
```

## 2. Scaffold the Rust service (robot-pi)
```bash
cd robot-pi
# Initialize a new binary crate
cargo init --bin .

# Create a placeholder README
cat > README.md << 'EOF'
# robot-pi
Rust-based WebSocket server, motor & sensor integration.

## Build
  cargo build --release

## Run
  # On Pi:
  ./target/release/robot-pi
EOF

# Commit scaffold
git add . && git commit -m "scaffold robot-pi crate"
cd ..
```

## 3. Scaffold the TypeScript UI (robot-web)
```bash
cd robot-web
# Initialize npm project
npm init -y

# Install dev dependencies
npm install --save-dev typescript lite-server

# Initialize TypeScript config
npx tsc --init --rootDir src --outDir dist --esModuleInterop --resolveJsonModule --lib es6,dom

# Create basic source structure
mkdir src
cat > src/index.html << 'EOF'
<!doctype html>
<html><head><meta charset="utf-8"><title>Robot UI</title></head><body>
  <h1>Robot Web UI</h1>
  <script src="main.js"></script>
</body></html>
EOF
cat > src/main.ts << 'EOF'
// entry-point: connect to WebSocket, bind WASD…
console.log('hello robot-web');
EOF

# Create a README for the UI
cat > README.md << 'EOF'
# robot-web
Browser-based UI (TypeScript + HTML/CSS).

## Install
  npm install

## Dev
  npm run start   # launches lite-server, serves dist/
EOF

# Update package.json scripts
npm pkg set scripts.build="tsc"
npm pkg set scripts.start="npm run build && lite-server --baseDir=dist"

# Commit scaffold
git add . && git commit -m "scaffold robot-web TS UI"
cd ..
```

## 4. Verify local toolchain
```bash
rustc --version   # e.g. 1.xx.x
cargo --version   # e.g. 1.xx.x
node --version    # v14+
npm --version
```

## 5. (Optional) Prepare cross-compilation
```bash
# Add ARM target for RPi
rustup target add armv7-unknown-linux-gnueabihf

# Or install `cross` for seamless builds
cargo install cross
```

## 6. Raspberry Pi setup for `ffmpeg`
```bash
ssh pi@<robot-ip>
sudo apt-get update && sudo apt-get install -y ffmpeg
ffmpeg -version
```