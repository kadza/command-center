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
GITHUB_REPOSITORY=${GITHUB_REPOSITORY:-}

if [[ -z "$PI_HOST" ]]; then
  echo "Error: PI_HOST environment variable is not set."
  echo "Usage: PI_HOST=<host> [PI_USER=<user>] [SSH_KEY=<key>] $0"
  exit 1
fi

# Determine repository (owner/repo) from git remote if not set
if [[ -z "$GITHUB_REPOSITORY" ]]; then
  ORIGIN_URL=$(git config --get remote.origin.url)
  if [[ $ORIGIN_URL =~ github.com[:/](.*)/(.*)\.git ]]; then
    GITHUB_REPOSITORY="${BASH_REMATCH[1]}/${BASH_REMATCH[2]}"
  else
    echo "Error: Unable to detect repository (set GITHUB_REPOSITORY=<owner/repo>)"
    exit 1
  fi
fi

echo "Fetching latest successful workflow run for 'ci.yml' on 'main'..."
RUN_ID=$(gh run list --repo "$GITHUB_REPOSITORY" --workflow ci.yml --branch main --limit 1 --json databaseId,conclusion --jq '.[] | select(.conclusion=="success") | .databaseId' | head -n1)
if [[ -z "$RUN_ID" ]]; then
  echo "Error: No successful workflow run for 'ci.yml' on branch 'main'"
  exit 1
fi
echo "Using workflow run #$RUN_ID"

# Download artifacts
TMPDIR=$(mktemp -d)
echo "Downloading artifacts to $TMPDIR..."
gh run download --repo "$GITHUB_REPOSITORY" "$RUN_ID" --name robot-pi-arm-binary --dir "$TMPDIR/arm"
gh run download --repo "$GITHUB_REPOSITORY" "$RUN_ID" --name robot-web-dist --dir "$TMPDIR/web"

REMOTE_BASE="/home/$PI_USER/robot-control"

echo "Stopping robot-pi service on $PI_USER@$PI_HOST..."
ssh -i "$SSH_KEY" "$PI_USER@$PI_HOST" "sudo systemctl stop robot-pi || true"

echo "Copying robot-pi binary..."
scp -i "$SSH_KEY" "$TMPDIR/arm/robot-pi" "$PI_USER@$PI_HOST:$REMOTE_BASE/robot-pi/target/release/robot-pi"

echo "Copying web assets..."
scp -i "$SSH_KEY" -r "$TMPDIR/web/dist/"* "$PI_USER@$PI_HOST:$REMOTE_BASE/robot-web/dist/"

echo "Starting robot-pi service..."
ssh -i "$SSH_KEY" "$PI_USER@$PI_HOST" "sudo systemctl start robot-pi"

echo "Cleaning up..."
rm -rf "$TMPDIR"

echo "Deployment complete."