# Funicular

![Rust](https://github.com/vincentserpoul/funicular/workflows/Rust/badge.svg?branch=master)
[![Coverage Status](https://coveralls.io/repos/github/vincentserpoul/funicular/badge.svg?branch=master)](https://coveralls.io/github/vincentserpoul/funicular?branch=master)

Automate alpine deployment on SBCs (rpi0, rpi2, rpi3, rpi4...).

1. Configure and build on your powerful desktop
2. Save an immutable local backup (apkovl.tar.gz)
3. Burn your sdcard and run in memory on your SBC

## Provisioners

We used a narrow concept of [packer provisioners](https://www.packer.io/docs/provisioners/shell-local/), by only allowing a shell-local configuration in order to simplify the configurations.
Check the test or example folder to check some examples of provisioners.

## Todo

[ ] config file and target dir as params
[ ] docker logs visible
[ ] docker error visible
[ ] switch from latest-stable to v3.12
[ ] auto rm when we quit funicular
