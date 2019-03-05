KERNEL = build/kernel.elf
NAME = c_kernel

CC = gcc
ASM = nasm
LD = ld

LD = ld
LDSCRIPT = linker.ld
LDFLAGS = -m elf_i386 -n --gc-sections
ifeq ($(LDMAP),yes)
	LDFLAGS += -M
endif

ASMFLAGS = -f elf

### MAIN FLAGS ###

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

### SOURCES ###

IFLAGS = -Iinclude -Ilibft/includes -Ilibasm_i386/includes

# main
SRC_C_KERNEL = kmain watchdog low_memory_c
SRC_ASM_KERNEL = init multiboot cpu_features low_memory_assembly
VPATH = include src
HEADERS = grub.h i386_type.h early_paging.asm early_gdt.asm watchdog.h

# system
SRC_C_KERNEL += bios_services idt panic
SRC_ASM_KERNEL += gdt int8086 irq_cpu exit_panic
SRC_ASM_KERNEL += init_pic 8254_pit 8042_keyboard default_pic_isr align_stack
VPATH += src/system src/system/pic
HEADERS += system.h

# keyboard
SRC_C_KERNEL += process_keyboard
SRC_ASM_KERNEL += keymap
VPATH += src/keyboard

# text_user_interface
SRC_C_KERNEL += text_core
VPATH += src/text_user_interface
HEADERS += text_user_interface.h

# graphic
SRC_C_KERNEL += set_vbe graphic_putchar graphic_tools bmp_load
SRC_ASM_KERNEL += draw_line alpha
VPATH += src/vesa
HEADERS += vesa.h

# memory
MEM_PAGING_C = paging virtual_mem physical_mem buddy page_fault
MEM_PAGING_ASM = set_paging

MEM_KALLOC = ctor mem_syscall \
		size_fn \
		index \
		debug_show_alloc debug_display_nodes \
		node_custom_allocator node_custom_deallocator \
		cmp_range cmp_fixe \
		free_record \
		allocator \
		reallocator \
		deallocator \
		k_main_prototypes \
		trace \
		sizeof_object \
		alloc_btree_ctor alloc_btree_get_node alloc_btree_apply_infix alloc_btree_memory_move alloc_btree_atomics_op \
		alloc_btree_insert_rnb_node alloc_btree_delete_rnb_node alloc_btree_insert_strategy alloc_btree_rotation_node \
		alloc_btree_try_to_insert alloc_btree_family_node alloc_btree_get_last_valid alloc_btree_delete_node \
		alloc_btree_insert_node alloc_btree_swap_content alloc_btree_limit_items alloc_btree_delete_strategy \
		alloc_btree_get_neighbours

MEM_VALLOC = v_main_prototypes

SRC_C_KERNEL += $(MEM_PAGING_C) $(MEM_KALLOC) $(MEM_VALLOC)
SRC_ASM_KERNEL += $(MEM_PAGING_ASM)
VPATH += src/memory src/memory/k_helpers src/memory/k_helpers/alloc_btree src/memory/v_helpers
HEADERS += memory_manager.h main_headers.h alloc_btree.h dynamic_allocator.h

# tests
SRC_C_KERNEL += mem_test rand_test
SRC_ASM_KERNEL += mmx sse1_sse2 avx
VPATH += src/tests
HEADER += tests.h

# math
SRC_C_KERNEL += rand
SRC_ASM_KERNEL += 8087
VPATH += src/math
HEADERS += math.h

# kernel_io
SRC_C_KERNEL += write k_tty
SRC_ASM_KERNEL +=
VPATH += src/kernel_io
HEADERS += kernel_io.h

# images
SRC_ASM_KERNEL += asterix univers
VPATH += medias

OBJ_DIR := objs
OBJ_ASM_KERNEL	= $(addprefix $(OBJ_DIR)/, $(addsuffix .o, $(basename $(notdir $(SRC_ASM_KERNEL)))))
OBJ_C_KERNEL	= $(addprefix $(OBJ_DIR)/, $(addsuffix .o, $(basename $(notdir $(SRC_C_KERNEL)))))

.PHONY: all clean fclean common_clean re help exec $(KERNEL)

all: symbol_gen_exe build_libasm_i386 build_libft $(KERNEL)

symbol_gen_exe:
	gcc -Wextra -Wall autobuild/nm_map_gen.c -o nm_map_gen -g -O0 -fsanitize=address
	$(gen_symbol_map)

$(KERNEL): $(OBJ_ASM_KERNEL) $(OBJ_C_KERNEL) libft/libft.a libasm_i386/libasm_i386.a
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

	chmod -x $@

compile_panic = $(CC) -c $(CFLAGS) -o $(OBJ_DIR)/panic.o src/system/panic.c $(IFLAGS)
extract_kernel_symbols = nm -n $@ > nm.res
gen_symbol_map = ./nm_map_gen src/system/nm.map nm.res && sync
link_kernel = $(LD) $(LDFLAGS) -T $(LDSCRIPT) $^ -o $@

$(OBJ_DIR)/%.o: %.c Makefile $(HEADERS)
	$(CC) -c $(CFLAGS) -o $@ $< $(IFLAGS)

$(OBJ_DIR)/%.o: %.asm Makefile $(HEADERS)
	$(ASM) $(ASMFLAGS) -o $@ $<

build_libasm_i386:
	make -C libasm_i386 all

build_libft:
	make -C libft all DEBUG=$(DEBUG) OPTIM=$(OPTIM)

common_clean:
	rm -rvf $(OBJ_DIR)*/**
	rm -f nm.res
	rm -f nm_map_gen
	rm -f src/system/nm.map
	rm -f $(KERNEL)

clean: common_clean
	make -C libft/ clean
	make -C libasm_i386/ clean

fclean: common_clean
	make -C libft/ fclean
	make -C libasm_i386/ fclean

re: fclean all

help:
	@echo
	@echo "Kernel $(NAME)"
	@echo
	@echo "--------------------------------------------------------------------------"
	@echo " Disp rules."
	@echo
	@echo " all     : Compile the kernel $(NAME)."
	@echo " re      : Recompile all objets of the programs."
	@echo " clean   : Remove objects."
	@echo " fclean  : Remove objects and programs."
	@echo " help    : Display this."
	@echo "--------------------------------------------------------------------------"

exec:
	qemu-system-x86_64 -kernel $(KERNEL) -m 128M --enable-kvm -cpu IvyBridge
