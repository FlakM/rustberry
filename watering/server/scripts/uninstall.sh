#!/usr/bin/env bash
set -e

ssh -qt pi@192.168.0.100 '
    systemctl --user daemon-reload
    systemctl --user is-active watering_server.service || true
    systemctl --user disable watering_server.service && echo "disabled" || true
    systemctl --user stop watering_server.service && echo "stopped" || true
    rm "~/.config/systemd/user/watering_server*" || true
    rm ~/server/* -rf && echo "removed binaries" || true
    systemctl --user is-active watering_server.service || true
'