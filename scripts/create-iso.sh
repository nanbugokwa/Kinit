#!/bin/sh
IMG=$1
ISO=$2
mkdir -p iso/boot/grub
cp "$IMG" iso/boot/
cat > iso/boot/grub/grub.cfg <<EOF
set timeout=1
menuentry "simple-init" {
  linux /boot/vmlinuz init=/init
  initrd /boot/$(basename $IMG)
}
EOF
grub-mkrescue -o "$ISO" iso
