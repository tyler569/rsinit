# vim: noet ts=8 sw=8 sts=8

root.img: init sub makefile
	qemu-img create root.img 100M
	mkfs.ext2 root.img
	e2mkdir root.img:/dev
	e2mkdir root.img:/proc
	e2mkdir root.img:/root
	e2cp -P 755 init root.img:/
	e2cp -P 755 sub root.img:/root/

init: target/x86_64-unknown-linux-musl/debug/rsinit
	cp $< $@

sub: target/x86_64-unknown-linux-musl/debug/sub
	cp $< $@

target/x86_64-unknown-linux-musl/debug/rsinit: $(shell find src)
	cargo build
