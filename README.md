# Funicular

![Rust](https://github.com/vincentserpoul/funicular/workflows/Rust/badge.svg?branch=master)
[![Coverage Status](https://coveralls.io/repos/github/vincentserpoul/funicular/badge.svg?branch=master)](https://coveralls.io/github/vincentserpoul/funicular?branch=master)

Automate alpine deployment on SBCs (rpi0, rpi2, rpi3, rpi4...).

1. Configure and build on your powerful desktop
2. Save an immutable local backup (apkovl.tar.gz)
3. Burn your sdcard and run in memory on your SBC

## Requirements

- Linux (only linux is supported for now)
- Docker: funicular runs a container buildt by another project, (alpine-diskless-headless)[https://github.com/vincentserpoul/alpine-diskless-headless]

## Provisioners

We used a narrow concept of [packer provisioners](https://www.packer.io/docs/provisioners/shell-local/), by only allowing a shell-local configuration in order to simplify the configurations.
Check the test or example folder to check some examples of provisioners.

## Usage

### Download the linux executable from the latest release

(Here)[https://github.com/vincentserpoul/funicular/releases/download/v0.1.0/funicular-linux]

### Generate a default config

```bash
./funicular config gen
```

### Build and burn the out example

```bash
./funicular build -t ./out/example/target -c ./out/example/config.toml -H rpi -d /dev/sda -f true
```

It will build the example and burn it on /dev/sda for a rpi.
Next step: insert the card in a rpi and enjoy!

## Todo

    [ ] add a lot more docs
    [ ] switch from latest-stable to v3.12
