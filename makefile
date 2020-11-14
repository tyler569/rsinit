# vim: noet ts=8 sw=8 sts=8

root.img: init sub makefile
	sudo mkdir -p root
	qemu-img create root.img 100M
	mkfs.ext2 root.img
	sudo mount -o loop root.img root
	sudo mkdir -p root/dev
	sudo mkdir -p root/proc
	sudo mkdir -p root/root
	sudo cp init root
	sudo cp sub root/root
	sudo umount root

init: target/x86_64-unknown-linux-musl/debug/rinit
	cp $< $@

sub: target/x86_64-unknown-linux-musl/debug/sub
	cp $< $@

target/x86_64-unknown-linux-musl/debug/rinit: $(shell find src)
	cargo build
