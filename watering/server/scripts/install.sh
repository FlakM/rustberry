#!/usr/bin/env bash
set -e

ssh -qt pi@192.168.0.100 "
    mkdir -p /home/pi/server/
    systemctl --user daemon-reload
    systemctl --user enable watering_server.service
    systemctl --user start watering_server.service
    systemctl --user status watering_server.service
"