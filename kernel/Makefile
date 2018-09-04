NAME = kernel.elf

CC = gcc
ASM = nasm
LD = ld

### MAIN FLAGS ###

UNAME_S := $(shell uname -s)
ifeq ($(UNAME_S),Darwin)
ifeq ($(DEBUG),yes)
	CFLAGS = -m32 -std=gnu99 -Wextra -Wall -g -O0 -fno-builtin -fno-stack-protector -nostdlib -nodefaultlibs -fno-exceptions
else
	CFLAGS = -m32 -std=gnu99 -Wextra -Wall -fno-builtin -fno-stack-protector -nostdlib -nodefaultlibs -fno-exceptions
endif
endif

ifeq ($(UNAME_S),Linux)
ifeq ($(DEBUG),yes)
	CFLAGS = -m32 -std=gnu99 -Wextra -Wall -g -O0 -fno-builtin -fno-stack-protector -nostdlib -nodefaultlibs -fno-exceptions
else
	CFLAGS = -m32 -std=gnu99 -Wextra -Wall -fno-builtin -fno-stack-protector -nostdlib -nodefaultlibs -fno-exceptions
endif
endif

### LIBRAIRIES ###

LIB_DIR = .
_LIBFT = libft
LIBFT = $(addprefix $(LIB_DIR)/, $(_LIBFT))

### SOURCES ###

IFLAGS = -Iinclude -I$(LIBFT)/includes -I$(LIBFT)/srcs

VPATH = include src src/system src/graphic

LDFLAGS = -m elf_i386 -T link.ld

SRC_C_KERNEL = kmain shutdown_computer set_vbe putchar
SRC_ASM_KERNEL = _start gdt int8086 draw_line display_char

_HEADERS = i386_type vesa_graphic base_system

OBJ_DIR = objs

__OBJ_C_KERNEL__ = $(basename $(notdir $(SRC_C_KERNEL)))
OBJ_C_KERNEL = $(addprefix $(OBJ_DIR)/, $(addsuffix .o, $(__OBJ_C_KERNEL__)))

__OBJ_ASM_KERNEL__ = $(basename $(notdir $(SRC_ASM_KERNEL)))
OBJ_ASM_KERNEL = $(addprefix $(OBJ_DIR)/, $(addsuffix .o, $(__OBJ_ASM_KERNEL__)))

__H__ = $(basename $(notdir $(_HEADERS)))
HEADERS = $(addsuffix .h, $(__H__))

.PHONY: all clean fclean re help

all: $(NAME)

$(NAME): $(OBJ_ASM_KERNEL) $(OBJ_C_KERNEL) libft/libft.a
	ld $(LDFLAGS) $^ -o $@
	chmod -x $(NAME)

$(OBJ_DIR)/%.o: %.c $(HEADERS) Makefile
	$(CC) -c $(CFLAGS) -o $@ $< $(IFLAGS)

$(OBJ_DIR)/%.o: %.asm $(HEADERS) Makefile
	$(ASM) -f elf -o $@ $<

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
