.PHONY: setup image qemu
.EXPORT_ALL_VARIABLES:

setup:
	curl https://sh.rustup.rs -sSf | sh -s -- -y
	rustup install nightly
	rustup default nightly
	cargo install bootimage

# Compilation options
output = vga
kbd_layout = qwerty
build_mode = release

export PRESTIGE_KBD_LAYOUT = $(kbd_layout)

bin = target/x86_64-prestige/$(build_mode)/bootimage-prestige.bin
img = disk.img

cargo-args = --no-default-features --features $(output) --bin prestige

ifeq ($(build_mode), release)
	cargo-args += --release 
endif

image:
	qemu-img create $(img) 32M
	touch src/lib.rs 
	env | grep PRESTIGE
	cargo bootimage $(cargo-args)
	dd conv=notrunc if=$(bin) of=$(img)

qemu-args = -no-reboot -drive file=$(img),format=raw

ifeq ($(output), serial)
	qemu-args += -display none -chardev stdio,id=s0,signal=off -serial chardev:s0
endif

ifeq ($(build_mode), debug)
	qemu-args += -s -S
endif

qemu:
	qemu-system-x86_64 $(qemu-args)
clean:
	cargo clean