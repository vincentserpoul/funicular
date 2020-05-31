#!/usr/bin/env sh

set -euo

#==============================================================================#

#===============================  e n v  v a r s  =============================#

: "${BUILD_HOSTNAME?Need to set BUILD_HOSTNAME}"
: "${SSID?Need to set SSID}"
: "${PASSWORD?Need to set PASSWORD}"

#==============================  e t h e r n e t  ============================#

alpine_setup_wlan() {
    BUILD_HOSTNAME=$1
    SSID=$2
    PASSWORD=$3

    apk add wpa_supplicant
    rc-update add wpa_supplicant default

    wpa_passphrase "$SSID" "$PASSWORD" >/etc/wpa_supplicant/wpa_supplicant.conf

    # remove the clear password...
    sed -i '/^[[:blank:]]*#/d;s/#.*//' /etc/wpa_supplicant/wpa_supplicant.conf

    cat <<EOF >>/etc/network/interfaces
auto wlan0
iface wlan0 inet dhcp
    hostname $BUILD_HOSTNAME

EOF
}

#==============================================================================#

alpine_setup_wlan "$BUILD_HOSTNAME" "$SSID" "$PASSWORD"
