# Robot Control System Specification

## Project Overview

The goal is to build a command center to remotely control a two-wheeled robot via a web interface. The robot will be controlled using the WASD keys for basic movement, with a live video feed and position tracking displayed on a dynamic map. Communication will happen through WebSockets, and video streaming will be handled using an external tool (`ffmpeg`). The robot’s position will be updated every second from its GNSS RTK sensor and visualized on the map.

## System Requirements

### 1. Hardware

- **Robot:**

  - Raspberry Pi 2 (or equivalent) with Linux-based OS (e.g., Raspbian).
  - GNSS RTK sensor for precise location tracking.
  - Camera module (supports H.264 video encoding).
  - Distance sensor (optional).
  - Wi-Fi for communication with the PC.

- **PC:**
  - macOS-based system for the operator.
  - Web browser (Chrome, Safari, or Firefox recommended).

### 2. Software Stack

- **On the Robot (Raspberry Pi):**

  - Rust for communication, motor control, and sensor integration.
  - `ffmpeg` for video streaming (H.264).
  - WebSocket server for communication with the PC.

- **On the PC:**
  - Web-based GUI written in TypeScript.
  - WebSockets for real-time communication with the robot.
  - OpenStreetMap for dynamic map visualization.

## Functional Requirements

### 1. Robot Control

- **Movement Control**:
  - Basic movement controls using the WASD keys:
    - **W**: Move forward.
    - **S**: Move backward.
    - **A**: Turn left.
    - **D**: Turn right.
  - **Simultaneous Key Presses**: Support diagonal movement when two keys are pressed simultaneously (e.g., W + A for forward-left).

### 2. Video Feed

- **FPV-Style Streaming**:
  - Stream video in real-time using **H.264** format.
  - Video stream handled by **ffmpeg** on the Raspberry Pi.
  - Low-latency video for near-instant feedback on the GUI.

### 3. Position Tracking

- **GNSS RTK Sensor**:
  - The robot’s position will be obtained from the GNSS RTK sensor, which updates the position every second.
  - The position will be visualized on a **dynamic map** (OpenStreetMap) within the GUI.
  - The robot’s position will be updated automatically in real-time on the map.

### 4. GUI Features

- **Interface**:
  - A browser-based interface (TypeScript + HTML/CSS).
  - Displays the robot’s live video feed.
  - Displays the robot’s position on a dynamic OpenStreetMap.
  - **WASD controls** for manual movement.
  - **Stop Button**: A button to stop the robot’s movement instantly.

### 5. Safety Features

- **Stop Button**:
  - Immediately halts robot movement when pressed.
  - Automatically triggers an emergency shutdown if certain conditions are met (e.g., low battery, loss of connection).

### 6. Communication

- **WebSocket Protocol**:
  - Full-duplex communication between the robot (Raspberry Pi) and the PC.
  - WebSocket server on the robot sends movement commands, video feed, and position updates to the PC.
  - The PC sends movement commands (WASD) to the robot in real-time.

## System Architecture

### 1. Communication Flow

1. **Robot to PC**:

   - The Raspberry Pi runs a WebSocket server that transmits:
     - **Video Feed**: Encoded H.264 video streamed via `ffmpeg`.
     - **Position Updates**: Every second, the GNSS RTK sensor sends updated coordinates, which are forwarded to the PC.
   - The Raspberry Pi also listens for movement commands from the PC and processes them to control the robot’s motors.

2. **PC to Robot**:
   - The PC sends control commands via WebSocket (WASD keys) to the Raspberry Pi.
   - The movement commands trigger the corresponding motor actions on the robot.

### 2. Data Flow

- **Video**:
  - Streamed from the Raspberry Pi to the web interface via WebSocket.
  - Handled by `ffmpeg` to encode and compress the video.
- **Position**:
  - Sent as GPS coordinates (latitude, longitude) from the GNSS RTK sensor to the PC every second.
  - The position is displayed on the dynamic OpenStreetMap interface.

### 3. Error Handling and Safety Mechanisms

- **Connection Loss**:
  - If the WebSocket connection is lost, the PC should attempt a **manual reconnect** via the GUI.
- **Emergency Stop**:
  - The GUI will include a **Stop** button to immediately halt all movement commands.
  - The Raspberry Pi should handle emergency shutdown conditions (e.g., low battery, loss of GPS signal) and send status messages to the GUI.

## Testing Plan

### 1. Unit Testing

- **Robot Movement Commands**:
  - Test that each WASD key press results in the correct motor action.
  - Test simultaneous key presses for diagonal movement.
- **Position Updates**:

  - Validate that position updates are sent from the GNSS RTK sensor every second.
  - Ensure the robot’s position is correctly mapped on the OpenStreetMap.

- **WebSocket Communication**:
  - Test WebSocket connection reliability under various conditions (e.g., reconnecting, latency).
  - Verify that movement commands from the PC are received and processed correctly by the robot.

### 2. Integration Testing

- **Full Communication Flow**:

  - Test that the WebSocket server on the Pi properly sends video and position updates to the PC.
  - Ensure that the GUI displays video and position data in real-time.

- **Emergency Features**:
  - Test the **Stop** button to ensure immediate cessation of robot movement.
  - Simulate emergency conditions (e.g., low battery) and ensure the robot properly shuts down and sends alerts to the PC.

### 3. User Testing

- **User Interface Testing**:
  - Test that the WASD controls are intuitive and responsive.
  - Ensure the video feed latency is low enough for an FPV driving experience.
  - Confirm that the map auto-refreshes and shows the robot’s position accurately.

### 4. Stress Testing

- **High Latency and Reconnect**:
  - Simulate Wi-Fi signal loss and check that the manual reconnect feature works properly.
  - Stress-test the WebSocket connection with frequent commands and ensure reliable performance.

## Deployment Considerations

- Ensure that all software dependencies (e.g., Rust, `ffmpeg`, WebSocket server) are clearly documented for both the robot and PC.
- The Raspberry Pi should run a startup script to ensure the WebSocket server and video streaming process begin automatically on boot.
- Make the GUI accessible via a local network IP address for ease of connection.
  
## Manual Deployment

After your code is built by GitHub Actions, you can deploy the latest ARM binary and web assets to your Pi locally using the GitHub CLI and the provided script:

1. Make sure you have the GitHub CLI (`gh`) installed and authenticated (e.g. `gh auth login`).

2. Run the deploy script:
   ```bash
   PI_HOST=<pi_host_or_ip> [PI_USER=<pi_user>] [SSH_KEY=<path_to_ssh_key>] ./scripts/deploy.sh
   ```

By default:
  - `PI_USER` defaults to `pi`.
  - `SSH_KEY` defaults to `~/.ssh/id_rsa`.

The script will:
  - Fetch the latest successful run of the `ci.yml` workflow on the `main` branch.
  - Download the `robot-pi-arm-binary` artifact and the `robot-web-dist` artifact.
  - Copy them into `/home/<pi_user>/robot-control/robot-pi/target/release/robot-pi` and `/home/<pi_user>/robot-control/robot-web/dist/`.
  - Restart the `robot-pi` systemd service on the Raspberry Pi.
