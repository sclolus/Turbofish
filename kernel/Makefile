NAME = kernel.elf

CC = gcc
ASM = nasm
LD = ld

### MAIN FLAGS ###

UNAME_S := $(shell uname -s)
ifeq ($(UNAME_S),Darwin)
ifeq ($(DEBUG),yes)
	CFLAGS = -m32 -std=gnu99 -Wextra -Wall -g -O0
	CFLAGS += -fno-builtin -fno-stack-protector -nostdlib -nodefaultlibs -fno-exceptions
	CFLAGS += -nostdinc
else
	CFLAGS = -m32 -std=gnu99 -Wextra -Wall
	CFLAGS += -fno-builtin -fno-stack-protector -nostdlib -nodefaultlibs -fno-exceptions
	CFLAGS += -nostdinc
endif
endif

ifeq ($(UNAME_S),Linux)
ifeq ($(DEBUG),yes)
	CFLAGS = -m32 -std=gnu99 -Wextra -Wall -g -O0
	CFLAGS += -fno-builtin -fno-stack-protector -nostdlib -nodefaultlibs -fno-exceptions
	CFLAGS += -nostdinc
else
	CFLAGS = -m32 -std=gnu99 -Wextra -Wall
	CFLAGS += -fno-builtin -fno-stack-protector -nostdlib -nodefaultlibs -fno-exceptions
	CFLAGS += -nostdinc
endif
endif

ASMFLAGS = -f elf

LDFLAGS = -m elf_i386 -T link.ld

### LIBRAIRIES ###

LIB_DIR = .
_LIBFT = libft
LIBFT = $(addprefix $(LIB_DIR)/, $(_LIBFT))

### SOURCES ###

IFLAGS = -Iinclude -I$(LIBFT)/includes -I$(LIBFT)/srcs


# main
SRC_C_KERNEL = kmain
SRC_ASM_KERNEL = _start
VPATH = include src
HEADERS = grub i386_type

# system
SRC_C_KERNEL += bios_services idt
SRC_ASM_KERNEL += gdt int8086 pic irq
VPATH += src/system
HEADERS += base_system

# keyboard
SRC_C_KERNEL += process_keyboard
SRC_ASM_KERNEL += keymap
VPATH += src/keyboard

# graphic
SRC_C_KERNEL += set_vbe graphic_putchar
SRC_ASM_KERNEL += draw_line display_char
VPATH += src/graphic
HEADERS += vesa_graphic

# memory
MEM_PAGING_C = paging virtual_mem physical_mem buddy
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
		alloc_btree_ctor alloc_btree_get_node alloc_btree_apply_infix alloc_btree_memory_move alloc_btree_atomics_op \
		alloc_btree_insert_rnb_node alloc_btree_delete_rnb_node alloc_btree_insert_strategy alloc_btree_rotation_node \
		alloc_btree_try_to_insert alloc_btree_family_node alloc_btree_get_last_valid alloc_btree_delete_node \
		alloc_btree_insert_node alloc_btree_swap_content alloc_btree_limit_items alloc_btree_delete_strategy \
		alloc_btree_get_neighbours

MEM_VALLOC = v_main_prototypes

SRC_C_KERNEL += $(MEM_PAGING_C) $(MEM_KALLOC) $(MEM_VALLOC)
SRC_ASM_KERNEL += $(MEM_PAGING_ASM)
VPATH += src/memory src/memory/k_helpers src/memory/k_helpers/alloc_btree src/memory/v_helpers
HEADERS += memory_manager main_headers alloc_btree

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
SRC_C_KERNEL += write
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

all: $(NAME)

$(NAME): $(OBJ_ASM_KERNEL) $(OBJ_C_KERNEL) libft/libft.a
	ld $(LDFLAGS) $^ -o $@
	chmod -x $(NAME)

$(OBJ_DIR)/%.o: %.c Makefile $(_HEADERS)
	$(CC) -c $(CFLAGS) -o $@ $< $(IFLAGS)

$(OBJ_DIR)/%.o: %.asm Makefile $(_HEADERS)
	$(ASM) $(ASMFLAGS) -o $@ $<

libft/libft.a:
	make -C $(LIBFT)/ all DEBUG=$(DEBUG)

clean:
	make -C $(LIBFT)/ clean
	rm -f $(OBJ_C_KERNEL)
	rm -f $(OBJ_ASM_KERNEL)

fclean:
	make -C $(LIBFT)/ fclean
	rm -f $(OBJ_C_KERNEL)
	rm -f $(OBJ_ASM_KERNEL)
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
	qemu-system-x86_64 -kernel $(NAME)
