# export TURBOFISH_ROOT in you shell configuration file to execute this Makefile independently
include $(TURBOFISH_ROOT)/boilerplates.mk

arch := ivybridge-cpu
target := $(arch)
custom_target := $(arch).json
ifeq ($(DEBUG),yes)
	rust_archive = target/$(target)/debug/lib$(LIBMOD).a
else
	optim_level += --release
	rust_archive = target/$(target)/release/lib$(LIBMOD).a
endif

CARGO_XBUILD_VERSION := $(shell cat xbuild-toolchain)

.PHONY = all clean fclean re exec build_rust

all: .rust-toolchain .xbuild-toolchain build_rust

.rust-toolchain: rust-toolchain
	rustup component add rust-src
	rustup component add rustfmt
	echo "Updated" > .rust-toolchain

.xbuild-toolchain: xbuild-toolchain
	cargo install --version $(CARGO_XBUILD_VERSION) cargo-xbuild --force
	echo "Updated" > .xbuild-toolchain

build_rust:
	cargo xbuild $(optim_level) --target $(custom_target) --verbose
	make -C dummy
	make -C rtc
	make -C key

clean:
	cargo clean
	make -C dummy clean
	make -C rtc clean
	make -C key clean

fclean: clean

re: fclean all
