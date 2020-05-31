#!/usr/bin/env sh

set -euo

#==============================================================================#

#===============================  e n v  v a r s  =============================#

: "${BUILD_HOSTNAME?Need to set BUILD_HOSTNAME}"

#==============================  e t h e r n e t  ============================#

alpine_setup_eth() {
    BUILD_HOSTNAME=$1
    cat <<EOF >>/etc/network/interfaces

auto eth0
iface eth0 inet dhcp
    hostname $BUILD_HOSTNAME

EOF
}

#==============================================================================#

alpine_setup_eth "$BUILD_HOSTNAME"
