#!/usr/bin/env sh

set -eu

#==============================================================================#

echo "test" >/etc/udev/rules.d/00-test.txt
echo "$PROVISIONER_UDEV_TEST" >/testttt.txt
