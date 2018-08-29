NAME = kernel.img

CC = gcc
ASM = nasm
LD = ld

### MAIN FLAGS ###

UNAME_S := $(shell uname -s)
ifeq ($(UNAME_S),Darwin)
ifeq ($(DEBUG),yes)
	CFLAGS = -m32 -std=c99 -Wextra -Wall -g -O0 -fsanitize=address
else
	CFLAGS = -m32 -std=c99 -Wextra -Wall -Ofast
endif
endif

ifeq ($(UNAME_S),Linux)
ifeq ($(DEBUG),yes)
	CFLAGS = -m32 -std=c99 -Wextra -Wall -g -O0 -fsanitize=address
else
	CFLAGS = -m32 -std=c99 -Wextra -Wall -Ofast
endif
endif

IFLAGS = -Iinclude

LDFLAGS = --ignore-unresolved-symbol _GLOBAL_OFFSET_TABLE_ -melf_i386 --oformat binary -Ttext 10000

SRC_KERNEL = kernel screen
_HEADERS = types

SRC_ASM = src/asm_file.asm
OBJ_ASM = objs/asm_file.o

VPATH = src include
OBJ_DIR = objs

__OBJ_KERNEL__ = $(basename $(notdir $(SRC_KERNEL)))
OBJ_KERNEL = $(addprefix $(OBJ_DIR)/, $(addsuffix .o, $(__OBJ_KERNEL__)))

__H__ = $(basename $(notdir $(_HEADERS)))
HEADERS = $(addsuffix .h, $(__H__))

.PHONY: all clean fclean re help

all: $(NAME)

$(NAME): $(OBJ_KERNEL) $(OBJ_ASM)
	ld $(LDFLAGS) $^ -o $@
	chmod -x $(NAME)

$(OBJ_DIR)/%.o: %.c $(HEADERS) Makefile
	$(CC) -c $(CFLAGS) -o $@ $< $(IFLAGS)

$(OBJ_ASM): $(SRC_ASM)
	$(ASM) -f elf $(SRC_ASM) -o $(OBJ_ASM)

clean:
	rm -f $(OBJ_KERNEL)
	rm -f $(OBJ_ASM)

fclean:
	rm -f $(OBJ_KERNEL)
	rm -f $(OBJ_ASM)
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