
arch := ivybridge_cpu
kernel := build/kernel.elf
name := undefined_behavior
# These rustc flags are defined in json xbuild file.
rustc_flags := -C no-redzone -C target-feature=-mmx,-sse,-sse2,-sse3,-ssse3,-sse4.1,-sse4.2,-3dnow,-3dnowa,-avx,-avx2

ASM = nasm
LD = ld
CC = gcc

### C FLAGS CONFIGURATION ###

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

ASMFLAGS = -f elf

# C debuging Rust
ifeq ($(DEV), C)
	SRC_C_KERNEL += debug_center vga_text
endif

SRC_ASM_KERNEL += cpuid init multiboot gdt
VPATH += src/boot

SRC_ASM_KERNEL += vbe_font
VPATH += src/monitor

SRC_ASM_KERNEL += real_mode_call
VPATH += src/system

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

linker_script := linker.ld

target := $(arch)
custom_target := $(arch).json
#$ come from this output: rustc -Z unstable-options --print target-spec-json --target i686-unknown-linux-gnu
#doc about this flags: https://github.com/rust-lang/rust/blob/256e497fe63bf4b13f7c0b58fa17360ca849c54d/src/librustc_back/target/mod.rs#L228-L409 

ifeq ($(DEBUG),yes)
rust_os := target/$(target)/debug/lib$(name).a
else
cargo_flags += --release
rust_os := target/$(target)/release/lib$(name).a
endif

.PHONY: all clean fclean re exec kernel

all: $(kernel)

clean:
ifeq ($(DEV), C)
	make -C $(LIBFT)/ clean
	rm -f $(OBJ_C_KERNEL)
endif
	rm -f $(OBJ_ASM_KERNEL)
	cargo clean

fclean:
ifeq ($(DEV), C)
	make -C $(LIBFT)/ fclean
	rm -f $(OBJ_C_KERNEL)
endif
	rm -f $(OBJ_ASM_KERNEL)
	cargo clean

re: fclean all

kernel: $(kernel)

ifeq ($(DEV), C)
$(kernel): build_rust $(rust_os) $(linker_script) $(OBJ_ASM_KERNEL) $(OBJ_C_KERNEL) libft/libft.a
	$(LD) --verbose -m elf_i386 -n --gc-sections -T $(linker_script) -o $(kernel) $(OBJ_ASM_KERNEL) $(OBJ_C_KERNEL) $(rust_os) libft/libft.a
else
$(kernel): build_rust $(rust_os) $(linker_script) $(OBJ_ASM_KERNEL)
	$(LD) --verbose -m elf_i386 -n --gc-sections -T $(linker_script) -o $(kernel) $(OBJ_ASM_KERNEL) $(rust_os)
endif

build_rust:
	cargo xbuild $(cargo_flags) --target $(custom_target) --verbose

$(OBJ_DIR)/%.o: %.asm Makefile $(_HEADERS)
	$(ASM) $(ASMFLAGS) -o $@ $<

ifeq ($(DEV), C)
$(OBJ_DIR)/%.o: %.c Makefile $(_HEADERS)
	$(CC) -c $(CFLAGS) -o $@ $< $(IFLAGS)

libft/libft.a:
	make -C $(LIBFT)/ all DEBUG=$(DEBUG) OPTIM=$(OPTIM)
endif

exec:
	qemu-system-x86_64 --enable-kvm -cpu IvyBridge -m 64M -kernel build/kernel.elf
