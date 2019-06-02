TARGET = signal

SRC_ASM += start
VPATH += src

SRC_ASM += 8087
VPATH += src/math

arch := ivybridge_cpu
target := $(arch)
custom_target := $(arch).json
ifeq ($(DEBUG),yes)
	rust_archive = target/$(target)/debug/lib$(TARGET).a
else
	cargo_flags += --release
	rust_archive = target/$(target)/release/lib$(TARGET).a
endif

CARGO_XBUILD_VERSION := $(shell cat xbuild-toolchain)

AS = nasm

ASFLAGS = -f elf
LDFLAGS = -T linker.ld -m elf_i386
ifeq ($(LDMAP),yes)
    LDFLAGS += -M
endif

OBJ_DIR = obj
OBJ_ASM = $(addprefix $(OBJ_DIR)/, $(addsuffix .o, $(basename $(notdir $(SRC_ASM)))))

.PHONY = all clean fclean re exec build_rust test test-all

all: .rust_toolchain .xbuild_toolchain build_rust build_libc $(TARGET)

.rust_toolchain: rust-toolchain
	rustup component add rust-src
	rustup component add rustfmt
	echo "Updated" > .rust_toolchain

.xbuild_toolchain: xbuild-toolchain
	cargo install --version $(CARGO_XBUILD_VERSION) cargo-xbuild --force
	echo "Updated" > .xbuild_toolchain

$(TARGET): $(OBJ_ASM) $(rust_archive) libc/libc.a linker.ld
	$(LD) $(LDFLAGS) -o $@ $^

$(OBJ_DIR)/%.o: %.asm Makefile
	$(AS) $(ASFLAGS) -o $@ $<

build_rust:
	cargo xbuild $(cargo_flags) --target $(custom_target) --verbose

build_libc:
	make -C libc all DEBUG=$(DEBUG) OPTIM=$(OPTIM)

test:
	cargo test --target i686-unknown-linux-gnu

clean:
	cargo clean
	find $(OBJ_DIR) -type f -name '*.o' -delete

fclean: clean
	rm -f $(TARGET)

re: fclean all

exec: all
	./$(TARGET)
