#!/bin/bash

openssl req -x509 -newkey rsa:4096 -keyout mitmproxy.key -out mitmproxy.cer -sha256 -days 700000 -nodes
