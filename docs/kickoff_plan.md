 # Kick-off Plan

Below is a high‑level roadmap to bootstrap development and validate the core control-feedback loop quickly:

1. Project scaffolding & environment
   • Create a mono‑repo (or two subfolders) named `robot-pi` (Rust) and `robot-web` (TypeScript).
   • Add an individual README to each with build/run instructions.
   • Verify local and Raspberry Pi environments:
     – Rust toolchain (stable + potential `wasm32-unknown-unknown`), cross‑compile if needed.
     – Node.js (v14+) with `npm`/`yarn` for the web UI.
     – `ffmpeg` installed on the Pi.
   • See `docs/scaffold_environment.md` for the full, detailed setup script and commands.

2. Define & validate the WebSocket protocol
   • Draft minimal JSON message formats:
     – `{ \"type\": \"cmd\", \"payload\": \"W\" }`
     – `{ \"type\": \"pos\", \"lat\":  ..., \"lon\": ... }`
     – (future) binary/video chunks for H.264 frames.
   • Build a Rust echo server (`tokio-tungstenite` or `warp`) to mirror incoming messages.
   • Create a simple web page to open a WS, send test JSON, and log responses.

3. Motor control implementation (COMPLETED)
   • Added `robot-pi/src/motor.rs` with `MotorController` using `rppal` to drive L293D IN pins.
     – Pin pairs: (17,18) for Motor A; (27,22) for Motor B. EN1/EN2 tied high on the driver.
   • Integrated into `robot-pi/src/main.rs`:
     – Shared controller wrapped in `Arc<AsyncMutex<MotorController>>`.
     – WS cmd messages (`"W"`,`"A"`,`"S"`,`"D"`,`"STOP"`) invoke `forward()`, `turn_left()`, `backward()`, `turn_right()`, `stop()` respectively.
   • Provided a no-op stub on non-Linux platforms for local development.
   • UI WASD binding in `robot-web/src/main.ts` remains to send JSON cmd messages to trigger motor actions.

4. Position updates
   • Simulate GNSS RTK feed in Rust: send `{ \"type\": \"pos\", … }` with dummy coords every second.
   • In the web UI, render these coords on a basic map (e.g. Leaflet + OSM).

5. Video streaming integration
   • Stream H.264 from the Pi camera via `ffmpeg` into a pipe or UDP port.
   • In Rust, spawn `ffmpeg` and forward frames over WS (or a separate channel).
   • In the UI, use `<video>` or a JS H.264 decoder to display the feed.

6. Safety & reconnection
   • Add a “STOP” button in the UI sending `{ \"type\": \"cmd\", \"payload\": \"STOP\" }`.
   • In Rust, implement immediate motor shutdown on “STOP”.
   • Add WS onclose handlers in the UI with backoff-based auto-reconnect.

7. Testing & CI
   • Unit tests in Rust for command parsing and motor wrapper.
   • End‑to‑end tests (e.g. Puppeteer) for UI vs. a fake WS server.
   • CI pipeline (GitHub Actions/GitLab CI) to build both sides and run tests on each commit.