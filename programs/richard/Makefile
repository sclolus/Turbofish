TARGET = richard

SRC_ASM += start
VPATH += src

arch := ivybridge_cpu
target := $(arch)
custom_target := $(arch).json
cargo_flags += --release
rust_archive := target/$(target)/release/lib$(TARGET).a

CARGO_XBUILD_VERSION := $(shell cat xbuild-toolchain)

AS = nasm

ASFLAGS = -f elf
LDFLAGS = -T linker.ld -m elf_i386
ifeq ($(LDMAP),yes)
    LDFLAGS += -M
endif

OBJ_DIR = obj
OBJ_ASM = $(addprefix $(OBJ_DIR)/, $(addsuffix .o, $(basename $(notdir $(SRC_ASM)))))

.PHONY = all clean fclean re exec build_rust

all: .rust_toolchain .xbuild_toolchain build_rust $(TARGET)

.rust_toolchain: rust-toolchain
	rustup component add rust-src
	rustup component add rustfmt
	echo "Updated" > .rust_toolchain

.xbuild_toolchain: xbuild-toolchain
	cargo install --version $(CARGO_XBUILD_VERSION) cargo-xbuild --force
	echo "Updated" > .xbuild_toolchain

$(TARGET): $(OBJ_ASM) $(rust_archive)
	$(LD) $(LDFLAGS) -o $@ $^

$(OBJ_DIR)/%.o: %.asm Makefile
	$(AS) $(ASFLAGS) -o $@ $<

build_rust:
	cargo xbuild $(cargo_flags) --target $(custom_target) --verbose

clean:
	cargo clean
	find $(OBJ_DIR) -type f -name '*.o' -delete

fclean: clean
	rm -f $(TARGET)

re: fclean all

exec: all
	./$(TARGET)
