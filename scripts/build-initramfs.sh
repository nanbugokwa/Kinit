#!/bin/sh
INIT=$1
OUT=$2
mkdir -p initramfs/{bin,dev,etc,proc,sys,usr/bin}
cp "$INIT" initramfs/init
# Add busybox if available
if command -v busybox >/dev/null; then
  cp "$(command -v busybox)" initramfs/bin/
fi
cd initramfs && find . | cpio -o -H newc | gzip > "../$OUT"
