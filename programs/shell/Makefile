TARGET = shell

# --- Preserve this above part. It is stricly system ---
ifeq ($(GNU),yes)
SRC_ASM +=
else
SRC_ASM += start
endif

VPATH += src

AS = nasm
CC = gcc

ifeq ($(GNU),yes)
CFLAGS += -m32
# Add GNU libc includes
LDFLAGS = -m elf_i386 -dynamic-linker /lib/ld-linux.so.2 /usr/lib32/crt1.o /usr/lib32/crti.o /usr/lib32/crtn.o -lc
else
CFLAGS += -m32 -fno-builtin -fno-stack-protector -nostdlib -nodefaultlibs -fno-exceptions -nostdinc
LDFLAGS = -T linker.ld -m elf_i386
endif
ASFLAGS = -f elf
CFLAGS += -m32
ifeq ($(LDMAP),yes)
    LDFLAGS += -M
endif

ifeq ($(GNU),yes)
	IFLAGS +=
else
	IFLAGS += -Ilibc/include
endif

BUILD_DIR = build
OBJ_DIR = obj
OBJ_ASM = $(addprefix $(OBJ_DIR)/, $(addsuffix .o, $(basename $(notdir $(SRC_ASM)))))
OBJ_C = $(addprefix $(OBJ_DIR)/, $(addsuffix .o, $(basename $(notdir $(SRC_C)))))

.PHONY = all clean fclean re exec build_libc build_rust test

# Rust stuff here:
arch := ivybridge-cpu
target := $(arch)
custom_target := $(arch).json
ifeq ($(DEBUG),yes)
	rust_archive = target/$(target)/debug/lib$(TARGET).a
else
	cargo_flags += --release
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

ifeq ($(GNU),yes)
all: .rust-toolchain .xbuild-toolchain build_rust $(TARGET)
else
all: .rust-toolchain .xbuild-toolchain build_rust build_libc $(TARGET)
endif

ifeq ($(GNU),yes)
$(TARGET): $(OBJ_ASM) $(OBJ_C) $(rust_archive)
	$(LD) $(LDFLAGS) -o $(BUILD_DIR)/$@ $^
else
$(TARGET): $(OBJ_ASM) $(OBJ_C) $(rust_archive) libc/libc.a
	$(LD) $(LDFLAGS) -o $(BUILD_DIR)/$@ $^
endif

$(OBJ_DIR)/%.o: %.asm Makefile $(HEADERS)
	$(AS) $(ASFLAGS) -o $@ $<

$(OBJ_DIR)/%.o: %.c Makefile $(HEADERS)
	$(CC) -c $(CFLAGS) $(IFLAGS) -o $@ $<

clean:
	cargo clean
	find $(OBJ_DIR) -type f -name '*.o' -delete

fclean: clean
	rm -f $(TARGET)

re: fclean all

build_rust:
	cargo xbuild $(cargo_flags) --target $(custom_target) --verbose

build_libc:
	make -C libc all DEBUG=$(DEBUG) OPTIM=$(OPTIM)

test:
	cargo test --target i686-unknown-linux-gnu

exec: all
	./$(TARGET)
