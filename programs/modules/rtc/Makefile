# export TURBOFISH_ROOT in you shell configuration file to execute this Makefile independently
include $(TURBOFISH_ROOT)/boilerplates.mk

BUILD_DIR = mod

LIBMOD = rtc
MODULE = $(addprefix $(BUILD_DIR)/, $(addsuffix .mod, $(basename $(notdir $(LIBMOD)))))

VPATH += src
SRC_ASM +=
SRC_C +=

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

AS = i686-turbofish-as
CC = i686-turbofish-gcc
LD = i686-turbofish-ld
NASM = nasm

ASFLAGS = -f elf
LDFLAGS = -T linker.ld -m elf_i386
ifeq ($(LDMAP),yes)
    LDFLAGS += -M
endif

OBJ_DIR = obj
OBJ_ASM = $(addprefix $(OBJ_DIR)/, $(addsuffix .o, $(basename $(notdir $(SRC_ASM)))))
OBJ_C = $(addprefix $(OBJ_DIR)/, $(addsuffix .o, $(basename $(notdir $(SRC_C)))))

.PHONY = all clean fclean re exec build_rust

all: .rust-toolchain .xbuild-toolchain build_rust $(MODULE)

$(MODULE): $(rust_archive)
	$(LD) $(LDFLAGS) -o $@ $^
	cp -rv --parents $@ $(SYSTEM_ROOT)/turbofish/

.rust-toolchain: rust-toolchain
	rustup component add rust-src
	rustup component add rustfmt
	echo "Updated" > .rust-toolchain

.xbuild-toolchain: xbuild-toolchain
	cargo install --version $(CARGO_XBUILD_VERSION) cargo-xbuild --force
	echo "Updated" > .xbuild-toolchain

$(OBJ_DIR)/%.o: %.asm Makefile
	$(NASM) $(ASFLAGS) -o $@ $<

$(OBJ_DIR)/%.o: %.as Makefile
	$(AS) $(ASFLAGS) -o $@ $<

$(OBJ_DIR)/%.o: %.c Makefile
	$(CC) $(ASFLAGS) -o $@ $<

build_rust:
	cargo xbuild $(optim_level) --target $(custom_target) --verbose

clean:
	cargo clean
	find $(OBJ_DIR) -type f -name '*.o' -delete

fclean: clean
	rm -f $(MODULE)

re: fclean all
