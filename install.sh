#!/usr/bin/env bash
set -e

nice cargo build --release

# systemd
sudo mkdir -p /usr/local/lib/systemd/system/
sudo cp -uv ./*.service /usr/local/lib/systemd/system/
sudo systemctl daemon-reload

# stop, replace and start new version
sudo systemctl stop ip-changed-telegram-message.service
sudo cp -v target/release/ip-changed-telegram-message /usr/local/bin
sudo systemctl enable --now ip-changed-telegram-message.service
