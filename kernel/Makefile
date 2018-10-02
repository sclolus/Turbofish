NAME = kernel.elf

CC = gcc
ASM = nasm
LD = ld

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

ASMFLAGS = -f elf

LDFLAGS = -m elf_i386 -T link.ld

### LIBRAIRIES ###

LIB_DIR = .
_LIBFT = libft
LIBFT = $(addprefix $(LIB_DIR)/, $(_LIBFT))

LIBASM_I386_DIR = .
_LIBASM_I386 = libasm_i386
LIBASM_I386 = $(addprefix $(LIBASM_I386_DIR)/, $(_LIBASM_I386))

### SOURCES ###

IFLAGS = -Iinclude -I$(LIBFT)/includes -I$(LIBFT)/srcs -I$(LIBASM_I386)/includes


# main
SRC_C_KERNEL = kmain
SRC_ASM_KERNEL = _start
VPATH = include src
HEADERS = grub i386_type

# system
SRC_C_KERNEL += bios_services idt panic
SRC_ASM_KERNEL += gdt int8086 irq_cpu exit_panic
SRC_ASM_KERNEL += init_pic 8254_pit 8042_keyboard default_pic_isr
VPATH += src/system src/system/pic
HEADERS += system

# keyboard
SRC_C_KERNEL += process_keyboard
SRC_ASM_KERNEL += keymap
VPATH += src/keyboard

# text_user_interface
SRC_C_KERNEL += text_core
VPATH += src/text_user_interface
HEADERS += text_user_interface

# graphic
SRC_C_KERNEL += set_vbe graphic_putchar graphic_tools bmp_load
SRC_ASM_KERNEL += draw_line alpha
VPATH += src/vesa
HEADERS += vesa

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
HEADERS += memory_manager main_headers alloc_btree dynamic_allocator

# tests
SRC_C_KERNEL += mem_test rand_test
VPATH += src/tests
HEADER += tests

# math
SRC_C_KERNEL += rand
SRC_ASM_KERNEL += 8087
VPATH += src/math
HEADERS += math

# kernel_io
SRC_C_KERNEL += write k_tty
SRC_ASM_KERNEL +=
VPATH += src/kernel_io
HEADERS += kernel_io

OBJ_DIR = objs

__OBJ_ASM_KERNEL__ = $(basename $(notdir $(SRC_ASM_KERNEL)))
OBJ_ASM_KERNEL = $(addprefix $(OBJ_DIR)/, $(addsuffix .o, $(__OBJ_ASM_KERNEL__)))

__OBJ_C_KERNEL__ = $(basename $(notdir $(SRC_C_KERNEL)))
OBJ_C_KERNEL = $(addprefix $(OBJ_DIR)/, $(addsuffix .o, $(__OBJ_C_KERNEL__)))

__H__ = $(basename $(notdir $(HEADERS)))
_HEADERS = $(addsuffix .h, $(__H__))

.PHONY: all clean fclean re help

all: pre-build target

target: $(OBJ_ASM_KERNEL) $(OBJ_C_KERNEL) $(OBJ_DIR)/univers.o $(OBJ_DIR)/asterix.o libft/libft.a libasm_i386/libasm_i386.a
	@echo PREBUILD
	ld $(LDFLAGS) $^ -o $(NAME)

	nm -n $(NAME) > nm.res
	./nm_map_gen src/system/nm.map nm.res
	sync
	$(CC) -c $(CFLAGS) -o $(OBJ_DIR)/panic.o src/system/panic.c $(IFLAGS)
	@echo MAINBUILD
	ld $(LDFLAGS) $^ -o $(NAME)

	nm -n $(NAME) > nm.res
	./nm_map_gen src/system/nm.map nm.res
	sync
	$(CC) -c $(CFLAGS) -o $(OBJ_DIR)/panic.o src/system/panic.c $(IFLAGS)
	@echo POSTBUILD
	ld $(LDFLAGS) $^ -o $(NAME)

	chmod -x $(NAME)

pre-build:
	gcc -Wextra -Wall autobuild/nm_map_gen.c -o nm_map_gen -g -O0 -fsanitize=address
	./nm_map_gen src/system/nm.map
	sync

$(OBJ_DIR)/%.o: %.c Makefile $(_HEADERS)
	$(CC) -c $(CFLAGS) -o $@ $< $(IFLAGS)

$(OBJ_DIR)/%.o: %.asm Makefile $(_HEADERS)
	$(ASM) $(ASMFLAGS) -o $@ $<

libft/libft.a:
	make -C $(LIBFT)/ all DEBUG=$(DEBUG) OPTIM=$(OPTIM)

libasm_i386/libasm_i386.a:
	make -C $(LIBASM_I386)/ all DEBUG=$(DEBUG) OPTIM=$(OPTIM)

$(OBJ_DIR)/univers.o: medias/univers.bmp
	ld $(LDFLAGS) -r -b binary $< -o $@

$(OBJ_DIR)/asterix.o: medias/asterix.bmp
	ld $(LDFLAGS) -r -b binary $< -o $@

clean:
	make -C $(LIBFT)/ clean
	make -C $(LIBASM_I386)/ clean
	rm -f $(OBJ_C_KERNEL)
	rm -f $(OBJ_ASM_KERNEL)
	rm -f $(OBJ_DIR)/univers.o
	rm -f $(OBJ_DIR)/asterix.o
	rm -f nm_map_gen nm.res src/system/nm.map

fclean:
	make -C $(LIBFT)/ fclean
	make -C $(LIBASM_I386)/ fclean
	rm -f $(OBJ_C_KERNEL)
	rm -f $(OBJ_ASM_KERNEL)
	rm -f $(OBJ_DIR)/univers.o
	rm -f $(OBJ_DIR)/asterix.o
	rm -f nm_map_gen nm.res src/system/nm.map
	rm -f $(NAME)

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
	qemu-system-x86_64 -kernel $(NAME) --enable-kvm
