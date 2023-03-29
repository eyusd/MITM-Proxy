#!/bin/bash

OLD_PROXY_MODE=$(gsettings get org.gnome.system.proxy mode)
OLD_HTTP_PROXY_HOST=$(gsettings get org.gnome.system.proxy.http host)
OLD_HTTP_PROXY_PORT=$(gsettings get org.gnome.system.proxy.http port)
OLD_HTTPS_PROXY_HOST=$(gsettings get org.gnome.system.proxy.https host)
OLD_HTTPS_PROXY_PORT=$(gsettings get org.gnome.system.proxy.https port)

echo "Starting proxy ..."

PROXY_HOST=localhost
PROXY_PORT=8100

gsettings set org.gnome.system.proxy mode manual
gsettings set org.gnome.system.proxy.http host "$PROXY_HOST"
gsettings set org.gnome.system.proxy.http port "$PROXY_PORT"
gsettings set org.gnome.system.proxy.https host "$PROXY_HOST"
gsettings set org.gnome.system.proxy.https port "$PROXY_PORT"

# Prevent the parent from aborting on INT
trap 'echo "Resetting proxy..."' INT
cargo run --release; X=$?

gsettings set org.gnome.system.proxy mode "$OLD_PROXY_MODE"
gsettings set org.gnome.system.proxy.http host "$OLD_HTTP_PROXY_HOST"
gsettings set org.gnome.system.proxy.http port "$OLD_HTTP_PROXY_PORT"
gsettings set org.gnome.system.proxy.https host "$OLD_HTTPS_PROXY_HOST"
gsettings set org.gnome.system.proxy.https port "$OLD_HTTPS_PROXY_PORT"

echo Goodbye !
