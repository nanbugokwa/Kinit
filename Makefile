.PHONY: build-server build-desktop test initramfs iso install

build-server:
	cargo build --release --features "server"

build-desktop:
	cargo build --release --features "desktop,tpm,luks"

test:
	cargo test

initramfs:
	@mkdir -p build
	scripts/build-initramfs.sh target/release/simple-init build/initramfs.img

iso:
	@mkdir -p build
	scripts/create-iso.sh build/initramfs.img build/simple-init.iso

install:
	install -Dm755 target/release/simple-init $(DESTDIR)/sbin/init
