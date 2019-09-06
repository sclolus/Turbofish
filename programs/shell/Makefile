# export TURBOFISH_ROOT in you shell configuration file to execute this Makefile independently
include $(TURBOFISH_ROOT)/boilerplates.mk

TARGET = shell

VPATH += src

CC = i686-turbofish-gcc

SRC_C = termios

BUILD_DIR = bin
OBJ_DIR = obj
OBJ_C = $(addprefix $(OBJ_DIR)/, $(addsuffix .o, $(basename $(notdir $(SRC_C)))))

.PHONY = all clean fclean re exec build_libc build_rust test

# Rust stuff here:
arch := ivybridge-cpu
target := $(arch)
custom_target := $(arch).json
ifeq ($(DEBUG),yes)
	rust_archive = target/$(target)/debug/lib$(TARGET).a
else
	optim_level += --release
	rust_archive = target/$(target)/release/lib$(TARGET).a
endif

CARGO_XBUILD_VERSION := $(shell cat xbuild-toolchain)

.rust-toolchain: rust-toolchain
	rustup component add rust-src
	rustup component add rustfmt
	echo "Updated" > .rust-toolchain

.xbuild-toolchain: xbuild-toolchain
	cargo install --version $(CARGO_XBUILD_VERSION) cargo-xbuild --force
	echo "Updated" > .xbuild-toolchain

all: .rust-toolchain .xbuild-toolchain build_rust $(TARGET)

$(TARGET): $(OBJ_ASM) $(OBJ_C) $(rust_archive)
	$(CC) -o $(BUILD_DIR)/$@ $^ -z muldefs
	cp -v --parents $(BUILD_DIR)/$@ $(SYSTEM_ROOT)

$(OBJ_DIR)/%.o: %.c Makefile $(HEADERS)
	$(CC) -c $(CFLAGS) $(IFLAGS) -o $@ $<

clean:
	cargo clean
	find $(OBJ_DIR) -type f -name '*.o' -delete

fclean: clean
	rm -f $(TARGET)

re: fclean all

build_rust:
	cargo xbuild $(optim_level) --target $(custom_target) --verbose

test:
	cargo test --target i686-unknown-linux-gnu

exec: all
	./$(TARGET)
