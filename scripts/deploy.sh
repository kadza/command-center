#!/usr/bin/env bash
set -euo pipefail

# Deployment script for Raspberry Pi
#
# Requires environment variables:
#   PI_HOST: Raspberry Pi host (IP or DNS)
#   PI_USER: SSH username (default: pi)
#   SSH_KEY: path to SSH private key (default: ~/.ssh/id_rsa)

PI_HOST=${PI_HOST:-}
PI_USER=${PI_USER:-pi}
SSH_KEY=${SSH_KEY:-$HOME/.ssh/id_rsa}

if [[ -z "$PI_HOST" ]]; then
  echo "Error: PI_HOST environment variable is not set."
  echo "Usage: PI_HOST=<host> [PI_USER=<user>] [SSH_KEY=<key>] $0"
  exit 1
fi

REMOTE_BASE="/home/$PI_USER/robot-control"

echo "Stopping robot-pi service on $PI_USER@$PI_HOST..."
ssh -i "$SSH_KEY" "$PI_USER@$PI_HOST" "sudo systemctl stop robot-pi || true"

echo "Copying robot-pi binary..."
scp -i "$SSH_KEY" "robot-pi/target/armv7-unknown-linux-gnueabihf/release/robot-pi" "$PI_USER@$PI_HOST:$REMOTE_BASE/robot-pi/target/release/robot-pi"

echo "Copying web assets..."
scp -i "$SSH_KEY" -r "robot-web/dist/"* "$PI_USER@$PI_HOST:$REMOTE_BASE/robot-web/dist/"

echo "Starting robot-pi service..."
ssh -i "$SSH_KEY" "$PI_USER@$PI_HOST" "sudo systemctl start robot-pi"

echo "Deployment complete."