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

LDFLAGS = --ignore-unresolved-symbol _GLOBAL_OFFSET_TABLE_ -melf_i386 --oformat binary -Ttext 1000

SRC_KERNEL = kernel screen
_HEADERS = types

VPATH = src include
OBJ_DIR = objs

__OBJ_KERNEL__ = $(basename $(notdir $(SRC_KERNEL)))
OBJ_KERNEL = $(addprefix $(OBJ_DIR)/, $(addsuffix .o, $(__OBJ_KERNEL__)))

__H__ = $(basename $(notdir $(_HEADERS)))
HEADERS = $(addsuffix .h, $(__H__))

.PHONY: all clean fclean re help

all: $(NAME)

$(NAME): $(OBJ_KERNEL)
	ld $(LDFLAGS) $^ -o $@
	chmod -x $(NAME)

$(OBJ_DIR)/%.o: %.c $(HEADERS) Makefile
	$(CC) -c $(CFLAGS) -o $@ $< $(IFLAGS)

clean:
	rm -f $(OBJ_KERNEL)

fclean:
	rm -f $(OBJ_KERNEL)
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