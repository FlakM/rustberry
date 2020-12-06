# Instructions

## Dev setup

local build:

```bash
export DATABASE_URL="$CLEVER_PG"
cargo build
```

https://stackoverflow.com/questions/37375712/cross-compile-rust-openssl-for-raspberry-pi-2


```bash


docker build docker/ -t rust-water-builder
docker run --rm --user  "$(id -u)":"$(id -g)" \
    -e DATABASE_URL="$CLEVER_PG" \
    -v "$PWD":/source \
    rust-water-builder




scp target/arm-unknown-linux-gnueabihf/release/water-read pi@192.168.0.100:~/
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