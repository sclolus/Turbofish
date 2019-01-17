arch := i686
kernel := build/kernel.elf
name := undefined_behavior
cargo_flags := 
rustc_params := -C no-redzone -C inline-threshold=0 -Cforce-frame-pointers=yes -C target-feature=-3dnow,-3dnowa,-avx2

ASM = nasm
LD = ld

### C MAIN PARTS

CC = gcc

### C FLAGS CONFIGURATION ###

UNAME_S := $(shell uname -s)
ifeq ($(UNAME_S),Darwin)
ifeq ($(DEBUG),yes)
	CFLAGS = -m32 -std=gnu99 -Wextra -Wall -g -O0
else ifeq ($(OPTIM),yes)
	# -fno-omit-frame-pointer keep good call trace for panic()
	CFLAGS = -m32 -std=gnu99 -Wextra -Wall -O2 -fno-omit-frame-pointer
else
	CFLAGS = -m32 -std=gnu99 -Wextra -Wall
endif
CFLAGS += -fno-builtin -fno-stack-protector -nostdlib -nodefaultlibs -fno-exceptions
CFLAGS += -nostdinc
endif

ifeq ($(UNAME_S),Linux)
ifeq ($(DEBUG),yes)
	CFLAGS = -m32 -std=gnu99 -Wextra -Wall -g -O0
else ifeq ($(OPTIM),yes)
	# -fno-omit-frame-pointer keep good call trace for panic()
	CFLAGS = -m32 -std=gnu99 -Wextra -Wall -O3 -fno-omit-frame-pointer
else
	CFLAGS = -m32 -std=gnu99 -Wextra -Wall
endif
CFLAGS += -fno-builtin -fno-stack-protector -nostdlib -nodefaultlibs -fno-exceptions
CFLAGS += -nostdinc
endif

ASMFLAGS = -f elf

# C debuging Rust
SRC_C_KERNEL += debug_center
SRC_ASM_KERNEL += cpuid
VPATH += src/debug

OBJ_DIR = objs

__OBJ_ASM_KERNEL__ = $(basename $(notdir $(SRC_ASM_KERNEL)))
OBJ_ASM_KERNEL = $(addprefix $(OBJ_DIR)/, $(addsuffix .o, $(__OBJ_ASM_KERNEL__)))

__OBJ_C_KERNEL__ = $(basename $(notdir $(SRC_C_KERNEL)))
OBJ_C_KERNEL = $(addprefix $(OBJ_DIR)/, $(addsuffix .o, $(__OBJ_C_KERNEL__)))

### C LIBRAIRIES ###

LIB_DIR = ../c_kernel
_LIBFT = libft
LIBFT = $(addprefix $(LIB_DIR)/, $(_LIBFT))

IFLAGS = -Iinclude -I$(LIBFT)/includes

linker_script := src/linker.ld

assembly_source_files := $(wildcard src/asm/*.asm)
assembly_object_files := $(patsubst src/asm/%.asm, \
    build/asm/%.o, $(assembly_source_files))

target := $(arch)-unknown-linux-gnu

ifeq ($(DEBUG),yes)
rust_os := target/$(target)/debug/lib$(name).a
else
cargo_flags += --release
rust_os := target/$(target)/release/lib$(name).a
endif

.PHONY: all clean kernel

all: $(kernel)

clean:
	make -C $(LIBFT)/ clean
	rm -f $(OBJ_C_KERNEL)
	rm -f $(OBJ_ASM_KERNEL)
	cargo clean

fclean:
	make -C $(LIBFT)/ fclean
	rm -f $(OBJ_C_KERNEL)
	rm -f $(OBJ_ASM_KERNEL)
	cargo clean
	rm -rf build

re: fclean all

kernel: $(kernel)

$(kernel): build_rust $(rust_os) $(assembly_object_files) $(linker_script) $(OBJ_ASM_KERNEL) $(OBJ_C_KERNEL) libft/libft.a
	$(LD) -m elf_i386 -n --gc-sections -T $(linker_script) -o $(kernel) $(assembly_object_files) $(rust_os) $(OBJ_ASM_KERNEL) $(OBJ_C_KERNEL) libft/libft.a

build_rust:
	cargo rustc $(cargo_flags) --verbose --target $(target) -- $(rustc_params)

# compile assembly files
build/asm/%.o: src/asm/%.asm
	mkdir -p $(shell dirname $@)
	$(ASM) -felf $< -o $@

$(OBJ_DIR)/%.o: %.c Makefile $(_HEADERS)
	$(CC) -c $(CFLAGS) -o $@ $< $(IFLAGS)

$(OBJ_DIR)/%.o: %.asm Makefile $(_HEADERS)
	$(ASM) $(ASMFLAGS) -o $@ $<

libft/libft.a:
	make -C $(LIBFT)/ all DEBUG=$(DEBUG) OPTIM=$(OPTIM)
