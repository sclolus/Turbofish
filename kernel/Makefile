NAME = kernel.img

CC = gcc
ASM = nasm -f elf

### MAIN FLAGS ###

UNAME_S := $(shell uname -s)
ifeq ($(UNAME_S),Darwin)
ifeq ($(DEBUG),yes)
	CFLAGS = -m32 -std=c99 -Wextra -Wall -Werror -g -O0 -fsanitize=address
else
	CFLAGS = -m32 -std=c99 -Wextra -Wall -Werror -O0
endif
endif

ifeq ($(UNAME_S),Linux)
ifeq ($(DEBUG),yes)
	CFLAGS = -m32 -std=c99 -Wextra -Wall -Werror -g -O0 -fsanitize=address
else
	CFLAGS = -m32 -std=c99 -Wextra -Wall -Werror -O0
endif
endif

LD_FLAGS = --ignore-unresolved-symbol _GLOBAL_OFFSET_TABLE_ -melf_i386 --oformat binary -Ttext 1000

### LIBRAIRIES ###

#LIB_DIR = .
#_LIBFT = libft
#LIBFT = $(addprefix $(LIB_DIR)/, $(_LIBFT))

### SOURCES ###

C_MAIN = kernel
ASM_MAIN = char

C_SRC_LIST = $(C_MAIN)
ASM_SRC_LIST = $(ASM_MAIN)

VPATH = base graphic lib

## HEADERS

_HEADERS = types

C_SRC = $(addsuffix .c, $(C_SRC_LIST))
ASM_SRC = $(addsuffix .c, $(ASM_SRC_LIST))

SRCS = $(C_SRC) $(ASM_SRC)

OBJ_DIR = objs
__OBJ__ = $(basename $(notdir $(SRCS)))
OBJ = $(addprefix $(OBJ_DIR)/, $(addsuffix .o, $(__OBJ__)))

__H__ = $(basename $(notdir $(_HEADERS)))
HEADERS = $(addsuffix .h, $(__H__))

#IFLAGS = -Iinclude -I$(LIBFT)/includes -I$(LIBFT)/srcs
IFLAGS = -Iinclude

ifeq ($(UNAME_S),Darwin)
#LDFLAGS = -L$(LIBFT) -lft
endif

ifeq ($(UNAME_S),Linux)
#LDFLAGS = -L$(LIBFT) -lft
endif

.PHONY: all clean fclean re help

all: $(NAME)

$(NAME): $(OBJ)
#	make -C $(LIBFT)/ all DEBUG=$(DEBUG)
	ld $(LD_FLAGS) $^ -o $@

$(OBJ_DIR)/%.o: %.c Makefile $(HEADERS)
	$(CC) -c $(CFLAGS) -o $@ $< $(IFLAGS)

$(OBJ_DIR)/%.o: %.asm Makefile
	$(ASM) -o $@ $<

clean:
#	make -C $(LIBFT)/ clean
	rm -f $(OBJ)

fclean:
#	make -C $(LIBFT)/ fclean
	rm -f $(OBJ)
	rm -f $(NAME)

re: fclean all

help:
	@echo
	@echo "Programm $(NAME)"
	@echo
	@echo "--------------------------------------------------------------------------"
	@echo " Disp rules."
	@echo
	@echo " all     : Compile the programs $(NAME)."
	@echo " re      : Recompile all objets of the programs."
	@echo " clean   : Remove objects."
	@echo " fclean  : Remove objects and programs."
	@echo " help    : Display this."
	@echo "--------------------------------------------------------------------------"
