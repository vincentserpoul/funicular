#!/usr/bin/env sh

set -euo

#==============================================================================#

#===============================  e n v  v a r s  =============================#

: "${TOKEN_VALUE?Need to set TOKEN_VALUE}"
: "${K3S_ARGS?Need to set K3S_ARGS}"
: "${TAINTS?Need to set TAINTS}"
: "${LABELS?Need to set LABELS}"

#===================================  k 3 s  ==================================#

k3s_install_pkg() {
    apk add k3s
}

#==============================================================================#

k3s_install_pkg