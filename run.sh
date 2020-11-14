#!/bin/sh

qemu-system-x86_64 \
    -m 128M \
    -kernel vmlinuz \
    -drive file=root.img,if=none,format=raw,media=disk,id=r1 \
    -device virtio-blk,drive=r1 \
    -append "root=/dev/vda console=ttyS0,115200" \
    -serial stdio \
    -display none
