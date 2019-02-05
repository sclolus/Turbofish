arch := ivybridge_cpu
kernel := build/kernel.elf
name := undefined_behavior

# These rustc flags are defined in json xbuild file.
# rustc_flags := -C no-redzone -C target-feature=-mmx,-sse,-sse2,-sse3,-ssse3,-sse4.1,-sse4.2,-3dnow,-3dnowa,-avx,-avx2

ASM = nasm
CC = gcc

LD = ld
linker_script := linker.ld
LDFLAGS = -m elf_i386 -n --gc-sections

ASMFLAGS = -f elf

ifeq ($(DEBUG),yes)
	CFLAGS = -m32 -std=gnu99 -Wextra -Wall -g -O0
	ASMFLAGS += -g
else ifeq ($(OPTIM),yes)
# -fno-omit-frame-pointer keep good call trace for panic()
	CFLAGS = -m32 -std=gnu99 -Wextra -Wall -O3 -fno-omit-frame-pointer
else
	CFLAGS = -m32 -std=gnu99 -Wextra -Wall
endif

CFLAGS += -fno-builtin -fno-stack-protector -nostdlib -nodefaultlibs -fno-exceptions -nostdinc

IFLAGS = -Iinclude -Ilibft/includes -Ilibftasm_i386/includes

# C debuging Rust
ifeq ($(DEV), C)
	SRC_C_KERNEL += debug_center vga_text
	SRC_ASM_KERNEL += cpuid
	HEADERS += vga_text.h cpuid.h
	VPATH += src/debug
endif

SRC_ASM_KERNEL += init multiboot gdt rust_ld cpu_features
VPATH += src/boot

SRC_ASM_KERNEL += vbe_font
VPATH += src/monitor

SRC_C_KERNEL += panic
SRC_ASM_KERNEL += real_mode_call cpu_isr io isr align_stack
VPATH += src/system

SRC_ASM_KERNEL += idt
VPATH += src/interrupts

OBJ_DIR := objs
OBJ_ASM_KERNEL	= $(addprefix $(OBJ_DIR)/, $(addsuffix .o, $(basename $(notdir $(SRC_ASM_KERNEL)))))
OBJ_C_KERNEL	= $(addprefix $(OBJ_DIR)/, $(addsuffix .o, $(basename $(notdir $(SRC_C_KERNEL)))))

target := $(arch)
custom_target := $(arch).json
# $ come from this output: rustc -Z unstable-options --print target-spec-json --target i686-unknown-linux-gnu
# doc about this flags: https://github.com/rust-lang/rust/blob/256e497fe63bf4b13f7c0b58fa17360ca849c54d/src/librustc_back/target/mod.rs#L228-L409

ifeq ($(DEBUG),yes)
	rust_os := target/$(target)/debug/lib$(name).a
else
	cargo_flags += --release
	rust_os := target/$(target)/release/lib$(name).a
endif

.PHONY: all clean fclean common_clean re exec $(kernel)

all: symbol_gen_exe build_rust build_libasm_i386 build_libft $(kernel)

common_clean:
	rm -f $(OBJ_C_KERNEL)
	rm -f $(OBJ_ASM_KERNEL)
	rm -f nm.res
	rm -f nm_map_gen
	rm -f $(kernel)
	cargo clean

clean: common_clean
	make -C libft clean
	make -C libasm_i386 clean

fclean: common_clean
	make -C libft fclean
	make -C libasm_i386 fclean

re: fclean all

compile_panic = $(CC) -c $(CFLAGS) -o $(OBJ_DIR)/panic.o src/system/panic.c $(IFLAGS)
extract_kernel_symbols = nm -n $@ > nm.res
gen_symbol_map = ./nm_map_gen src/system/nm.map nm.res && sync
link_kernel = $(LD) $(LDFLAGS) -T $(linker_script) -o $@ $^

symbol_gen_exe:
	gcc -Wextra -Wall autobuild/nm_map_gen.c -o nm_map_gen -g -O0 -fsanitize=address
	$(gen_symbol_map)

$(kernel): $(OBJ_ASM_KERNEL) $(OBJ_C_KERNEL) $(rust_os) libft/libft.a libasm_i386/libasm_i386.a
	@echo PREBUILD
	$(link_kernel)
	$(extract_kernel_symbols)
	$(gen_symbol_map)
	$(compile_panic)

	@echo MAINBUILD
	$(link_kernel)
	$(extract_kernel_symbols)
	$(gen_symbol_map)
	$(compile_panic)

	@echo POSTBUILD
	$(link_kernel)

build_rust:
	cargo xbuild $(cargo_flags) --target $(custom_target) --verbose

build_libasm_i386:
	make -C libasm_i386 all

build_libft:
	make -C libft all DEBUG=$(DEBUG) OPTIM=$(OPTIM)

$(OBJ_DIR)/%.o: %.asm Makefile $(HEADERS)
	$(ASM) $(ASMFLAGS) -o $@ $<

$(OBJ_DIR)/%.o: %.c Makefile $(HEADERS)
	$(CC) -c $(CFLAGS) -o $@ $< $(IFLAGS)

exec:
	qemu-system-x86_64 --enable-kvm -cpu IvyBridge -m 64M -kernel build/kernel.elf
