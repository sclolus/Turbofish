include ../boilerplates.mk

arch := ivybridge-cpu
kernel := build/kernel.elf
name := turbo_fish
CARGO_XBUILD_VERSION := $(shell cat xbuild-toolchain)
QEMU_ARGS = --enable-kvm -cpu IvyBridge -m 128M -kernel $(kernel)
# These rustc flags are defined in json xbuild file.
# rustc_flags := -C no-redzone -C target-feature=-mmx,-sse,-sse2,-sse3,-ssse3,-sse4.1,-sse4.2,-3dnow,-3dnowa,-avx,-avx2

ASM = nasm
CC = gcc

LD = ld
linker_script := linker.ld
LDFLAGS = -m elf_i386 -n --gc-sections
ifeq ($(LDMAP),yes)
	LDFLAGS += -M
endif

ASMFLAGS = -f elf

### GDB CONFIG ###
GDB_TCP_PORT = 9000

ifeq ($(DEBUG),yes)
	CFLAGS = -m32 -std=gnu99 -Wextra -Wall -g -O0
	ASMFLAGS += -g
else ifeq ($(OPTIM),yes)
# -fno-omit-frame-pointer keep good call trace for panic()
	CFLAGS = -m32 -std=gnu99 -Wextra -Wall -O3 -fno-omit-frame-pointer
else
	CFLAGS = -m32 -std=gnu99 -Wextra -Wall
endif

CFLAGS += -fno-builtin -fno-stack-protector -nostdlib -nodefaultlibs -fno-exceptions -nostdinc -fno-pie

IFLAGS = -Iinclude -Ilibft/includes -Ilibasm_i386/includes -I./

SRC_ASM_KERNEL += init_kernel start cpu_features
HEADERS +=
VPATH += src/boot

SRC_C_KERNEL += vga_cpy
SRC_ASM_KERNEL += vbe_font
VPATH += dependencies/terminal/src/monitor

SRC_C_KERNEL += panic

SRC_ASM_KERNEL += real_mode_call align_stack i8086_payload
VPATH += src/system

SRC_ASM_KERNEL += apm_shutdown
VPATH += src/system/i8086_payloads

SRC_ASM_KERNEL += cpu_exceptions_isr default_isr
VPATH += src/system

SRC_ASM_KERNEL += pic_8259_isr
VPATH += src/drivers/pic_8259

SRC_ASM_KERNEL += univers wanggle
VPATH += medias

SRC_ASM_KERNEL += paging
VPATH += src/memory/mmu

SRC_ASM_KERNEL += taskmaster
VPATH += src/taskmaster

SRC_ASM_KERNEL += auto_preempt
VPATH += src/taskmaster/scheduler

SRC_ASM_KERNEL += test
VPATH += src/taskmaster/syscall

SRC_ASM_KERNEL += trampoline
VPATH += src/taskmaster/signal

SRC_ASM_KERNEL += asm_processes idle_process
VPATH += src/taskmaster/tests src/taskmaster/process

SRC_ASM_KERNEL += payload_13h_extended
VPATH += src/drivers/storage/bios_int13h

SRC_C_KERNEL += ctor mem_syscall \
	size_fn \
	index \
	node_custom_allocator node_custom_deallocator \
	cmp_range cmp_fixe \
	free_record \
	allocator \
	reallocator \
	deallocator \
	main_prototypes \
	sizeof_object \
	alloc_btree_ctor alloc_btree_get_node alloc_btree_apply_infix alloc_btree_memory_move alloc_btree_atomics_op \
	alloc_btree_insert_rnb_node alloc_btree_delete_rnb_node alloc_btree_insert_strategy alloc_btree_rotation_node \
	alloc_btree_try_to_insert alloc_btree_family_node alloc_btree_get_last_valid alloc_btree_delete_node \
	alloc_btree_insert_node alloc_btree_swap_content alloc_btree_limit_items alloc_btree_delete_strategy \
	alloc_btree_get_neighbours
VPATH += src/memory/allocator/c src/memory/allocator/c/alloc_btree
#HEADERS += include/stdlib.h main_headers.h alloc_btree_internal_header.h
HEADERS += main_headers.h alloc_btree_internal_header.h

OBJ_DIR := objs
OBJ_ASM_KERNEL	= $(addprefix $(OBJ_DIR)/, $(addsuffix .o, $(basename $(notdir $(SRC_ASM_KERNEL)))))
OBJ_C_KERNEL	= $(addprefix $(OBJ_DIR)/, $(addsuffix .o, $(basename $(notdir $(SRC_C_KERNEL)))))

target := $(arch)
custom_target := $(arch).json
# $ come from this output: rustc -Z unstable-options --print target-spec-json --target i686-unknown-linux-gnu
# doc about this flags: https://github.com/rust-lang/rust/blob/256e497fe63bf4b13f7c0b58fa17360ca849c54d/src/librustc_back/target/mod.rs#L228-L409

ifeq ($(serial-eprintln), yes)
cargo_flags += --features serial-eprintln
endif

ifeq ($(DEBUG),yes)
	rust_os := target/$(target)/debug/lib$(name).a
else
	cargo_flags += --release
	rust_os := target/$(target)/release/lib$(name).a
endif

.PHONY: all clean fclean common_clean re exec $(kernel)

all: .rust-toolchain .xbuild-toolchain compile_bootstrap build_rust build_libasm_i386 build_libft $(kernel)

.rust-toolchain: rust-toolchain
	rustup component add rust-src
	rustup component add rustfmt
	echo "Updated" > .rust-toolchain

.xbuild-toolchain: xbuild-toolchain
	cargo install --version $(CARGO_XBUILD_VERSION) cargo-xbuild --force
	echo "Updated" > .xbuild-toolchain

common_clean:
	find $(OBJ_DIR) -type f -name '*.o' -delete
	rm -f autobuild/nm_output
	rm -f autobuild/nm.map
	rm -f autobuild/nm_map_gen
	rm -f $(kernel)
	cargo clean

clean: common_clean
	make -C libft clean
	make -C libasm_i386 clean
	make -C src/boot/bootstrap clean

fclean: common_clean
	make -C libft fclean
	make -C libasm_i386 fclean
	make -C src/boot/bootstrap fclean

re: fclean all

compile_bootstrap:
	$(gen_symbol_map) autobuild/nm.map && sync
	$(compile_bootstrap)

compile_panic = $(CC) -c $(CFLAGS) -o $(OBJ_DIR)/panic.o src/system/panic.c $(IFLAGS)
compile_bootstrap = make -C src/boot/bootstrap OWNED_BY_KERNEL=yes
extract_kernel_symbols = nm -n $@ > autobuild/nm_output
gen_symbol_map = cargo run --release --manifest-path autobuild/nm_map_gen_rust/Cargo.toml
link_kernel = $(LD) $(LDFLAGS) -T $(linker_script) -o $@ $^

LIBS = libft/libft.a libasm_i386/libasm_i386.a

$(kernel): src/boot/bootstrap/build/bootstrap.o $(OBJ_ASM_KERNEL) $(OBJ_C_KERNEL) $(rust_os) $(LIBS)
	@echo PREBUILD
	$(link_kernel)
	$(extract_kernel_symbols)
	$(gen_symbol_map) autobuild/nm.map autobuild/nm_output && sync
	$(compile_panic)
	$(compile_bootstrap)

	@echo MAINBUILD
	$(link_kernel)
	$(extract_kernel_symbols)
	$(gen_symbol_map) autobuild/nm.map autobuild/nm_output && sync
	$(compile_panic)
	$(compile_bootstrap)

	@echo POSTBUILD
	$(link_kernel)

build_rust:
	cargo xbuild $(cargo_flags) --target $(custom_target) --verbose --features=$(features)

build_libasm_i386:
	make -C libasm_i386 all

build_libft:
	make -C libft all DEBUG=$(DEBUG) OPTIM=$(OPTIM)

test: .
	cargo test --all --target i686-unknown-linux-gnu --features std-print --exclude ext2

test-all: .
	cargo test --all --target i686-unknown-linux-gnu --features std-print --exclude ext2 --no-fail-fast

$(OBJ_DIR)/%.o: %.asm Makefile $(HEADERS)
	$(ASM) $(ASMFLAGS) -o $@ $<

$(OBJ_DIR)/%.o: %.c Makefile $(HEADERS)
	$(CC) -c $(CFLAGS) -o $@ $< $(IFLAGS)

# This feature need to disable hard drive drivers in kernel
exec_gdb:
	qemu-system-x86_64 -S -gdb tcp::$(GDB_TCP_PORT) $(QEMU_ARGS)
