#!/usr/bin/env bash
set -e

ssh -qt pi@192.168.0.100 '
    systemctl --user daemon-reload
    systemctl --user status water.timer || true
    systemctl --user disable water.timer && echo "disabled" || true
    systemctl --user stop water.timer && echo "stopped" || true
    rm "~/.config/systemd/user/water*" || true
    rm ~/rustberry/* -f && echo "removed binaries" || true
    systemctl --user status water.timer || true
'