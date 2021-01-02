# Instructions

## Dev setup

local build:

```bash
export DATABASE_URL="$CLEVER_PG"
cargo build
```

setup on host:

```bash
# env files
touch /home/pi/.water.env

# setup dir for systemd services
mkdir -p ~/.config/systemd/user/

# enable non root processes to run even when not logged in
sudo loginctl enable-linger pi
```

And then you might run:

```bash
# it wiil compile & uninstall services & upload via ssh & install services
./build.sh
```
