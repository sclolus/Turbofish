NAME = libft.a
CC = gcc

LIBDIR	= ./
SRCDIR	= srcs
INCDIR	= includes
OBJDIR	= objs
HEADER	= libft.h btree.h

ifeq ($(DEBUG),yes)
	CFLAGS = -m32 -std=c99 -Wextra -Wall -g -O0 -fsanitize=address -fno-builtin -I./$(INCDIR) -fno-stack-protector
else
	CFLAGS = -m32 -std=c99 -Wextra -Wall -O2 -fno-builtin -I./$(INCDIR) -fno-stack-protector
endif

SRC_USUALS = ft_memcpy ft_memset ft_bzero ft_strlen

#SRC_USUALS	= ft_memset ft_bzero ft_memcpy ft_memccpy ft_memmove ft_memchr ft_memcmp \
	ft_strlen ft_strcpy ft_strncpy ft_strcat ft_strncat ft_strlcat \
	ft_strchr ft_strrchr ft_strstr ft_strnstr ft_strcmp ft_strncmp \
	ft_atoi ft_isalpha ft_isdigit ft_isalnum ft_isascii ft_isprint ft_toupper ft_tolower \
	ft_strclr ft_striter ft_striteri \
	ft_strequ ft_strnequ \
	ft_putchar ft_putstr ft_putendl ft_putnbr ft_putchar_fd ft_putstr_fd \
	ft_putendl_fd ft_putnbr_fd ft_secure_atoi \
	ft_aligned_memcpy ft_aligned_bzero

SRC_PRINTF	= beacon op_main modifiers extract_args \
	s_string s_char s_numeric_u s_numeric_i \
	s_pointer s_logical_b s_logical_o s_logical_xmin s_logical_xmaj	\
	cast buffer

SRC_BTREE	= btree_apply_infix btree_apply_prefix btree_apply_suffix \
	btree_apply_by_level btree_get_node btree_level_count \
	btree_check_binary_tree btree_memory_move \
	btree_delete_node btree_delete_rnb_node btree_insert_node btree_insert_rnb_node \
	btree_limit_items btree_ctor btree_check_rnb_property btree_atomics_op \
	btree_rotation_node btree_family_node \
	btree_insert_strategy btree_delete_strategy \
	btree_smash_checker \
	btree_get_last_valid btree_get_neighbours btree_conditional_insert

SRC_CHAINED	= lst_new lst_del_one lst_del lst_add lst_iter lst_map lst_invert_rec \
	lst_push_front lst_push_back lst_pre_alloc lst_merge lst_create_elem lst_invert_it

SRC_SORT    = fusion_merge_tab fusion_merge_chain_list

#SRC_GNL     = gnl

USUALS_DIR	= usuals_tools
PRINTF_DIR  = ft_printf
BTREE_DIR   = binary_tree
CHAINED_DIR = chained_tools
SORT_DIR    = sort_tools
GNL_DIR		= gnl

SRC = 	$(addprefix $(USUALS_DIR)/, $(addsuffix .c, $(SRC_USUALS))) \
		$(addprefix $(PRINTF_DIR)/, $(addsuffix .c, $(SRC_PRINTF))) \
		$(addprefix $(BTREE_DIR)/, $(addsuffix .c, $(SRC_BTREE))) \
		$(addprefix $(CHAINED_DIR)/, $(addsuffix .c, $(SRC_CHAINED))) \
		$(addprefix $(SORT_DIR)/, $(addsuffix .c, $(SRC_SORT))) \
		$(addprefix $(GNL_DIR)/, $(addsuffix .c, $(SRC_GNL)))

TMP = $(basename $(notdir $(SRC)))
OBJ = $(addprefix $(OBJDIR)/, $(addsuffix .o, $(TMP)))

.PHONY: all re clean fclean help

all: $(NAME)

$(NAME): $(OBJ) includes/libft.h includes/btree.h includes/chained_tools.h srcs/gnl/private_gnl.h srcs/ft_printf/internal_printf.h
	ar rc $(NAME) $(OBJ)
	ranlib $(NAME)

$(OBJDIR)/%.o: $(SRCDIR)/$(USUALS_DIR)/%.c includes/libft.h
	$(CC) -c $(CFLAGS) -o $@ $<

$(OBJDIR)/%.o: $(SRCDIR)/$(PRINTF_DIR)/%.c srcs/ft_printf/internal_printf.h
	$(CC) -c $(CFLAGS) -o $@ $<

$(OBJDIR)/%.o: $(SRCDIR)/$(SORT_DIR)/%.c includes/libft.h
	$(CC) -c $(CFLAGS) -o $@ $<

$(OBJDIR)/%.o: $(SRCDIR)/$(BTREE_DIR)/%.c includes/btree.h srcs/binary_tree/btree_internal_header.h
	$(CC) -c $(CFLAGS) -o $@ $<

$(OBJDIR)/%.o: $(SRCDIR)/$(CHAINED_DIR)/%.c includes/chained_tools.h
	$(CC) -c $(CFLAGS) -o $@ $<

$(OBJDIR)/%.o: $(SRCDIR)/$(GNL_DIR)/%.c srcs/gnl/private_gnl.h
	$(CC) -c $(CFLAGS) -o $@ $<

clean:
	rm -vf $(OBJ)

fclean:
	rm -vf $(OBJ)
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
