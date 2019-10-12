# export TURBOFISH_ROOT in you shell configuration file to execute this Makefile independently
include $(TURBOFISH_ROOT)/boilerplates.mk

NAME = minilibx_turbofish.a
CC = i686-turbofish-gcc
AS = nasm
AR = i686-turbofish-ar
RANLIB = i686-turbofish-ranlib

CFLAGS = -Ofast -std=c99 -Wall -Wextra -fno-omit-frame-pointer
IFLAGS = -I./include -I src/bmp
ASFLAGS = -f elf

LIBDIR	= ./
SRCDIR	= src
OBJDIR	= obj

SRC_C = turbofish_mlx bmp_load
SRC_ASM = vbe_font

VPATH = src
VPATH += src/bmp

OBJ_C = $(addprefix $(OBJDIR)/, $(addsuffix .o, $(basename $(notdir $(SRC_C)))))
OBJ_ASM = $(addprefix $(OBJDIR)/, $(addsuffix .o, $(basename $(notdir $(SRC_ASM)))))

.PHONY: all re clean fclean help

all: $(NAME)

$(NAME): $(OBJ_C) $(OBJ_ASM)
	$(AR) rc $(NAME) $(OBJ_C) $(OBJ_ASM)
	$(RANLIB) $(NAME)

$(OBJDIR)/%.o: %.c include/turbofish_mlx.h
	$(CC) -c $(CFLAGS) $(IFLAGS) -o $@ $<

$(OBJDIR)/%.o: $(SRCDIR)/%.asm include/turbofish_mlx.h
	$(AS) $(ASFLAGS) -o $@ $<

clean:
	rm -vf $(OBJ_C) $(OBJ_ASM)

fclean:
	rm -vf $(OBJ_C) $(OBJ_ASM)
	rm -vf $(NAME)

re: fclean all

help:
	@echo
	@echo "Librairy $(NAME)"
	@echo
	@echo "--------------------------------------------------------------------------"
	@echo " Disp rules."
	@echo
	@echo " all     : Compile the librairy if a file has changed, stored in $(LIBDIR)."
	@echo " re      : Recompile all objets of the librairy."
	@echo " clean   : Remove objects."
	@echo " fclean  : Remove objects and executable."
	@echo " help    : Display this."
	@echo "--------------------------------------------------------------------------"
	@echo
