#!/usr/bin/env bash
set -e

ssh -qt pi@192.168.0.100 "
    systemctl --user daemon-reload
    systemctl --user enable water.timer
    systemctl --user start water.timer
    systemctl --user status water.timer
"