NAME = libft.a
CC = gcc

LIBDIR	= ./
SRCDIR	= srcs
INCDIR	= includes
OBJDIR	= objs
HEADER	= libft.h

ifeq ($(DEBUG),yes)
	CFLAGS = -Wall -Werror -Wextra -std=c99 -g -O0 -fsanitize=address -I./$(INCDIR)
else
	CFLAGS = -Wall -Werror -Wextra -std=c99 -I./$(INCDIR)
endif

SRC_USUALS	= ft_memset ft_bzero ft_memcpy ft_memccpy ft_memmove ft_memchr ft_memcmp \
	ft_strlen ft_strdup ft_strcpy ft_strncpy ft_strcat ft_strncat ft_strlcat \
	ft_strchr ft_strrchr ft_strstr ft_strnstr ft_strcmp ft_strncmp \
	ft_atoi ft_isalpha ft_isdigit ft_isalnum ft_isascii ft_isprint ft_toupper ft_tolower \
	ft_memalloc ft_memdel ft_strnew ft_strdel ft_strclr ft_striter ft_striteri ft_strmap \
	ft_strmapi ft_strequ ft_strnequ ft_strsub ft_strjoin ft_strtrim ft_strsplit ft_itoa \
	ft_putchar ft_putstr ft_putendl ft_putnbr ft_putchar_fd ft_putstr_fd \
	ft_putendl_fd ft_putnbr_fd ft_realloc ft_secure_atoi ft_fstrsub

SRC_PRINTF	= beacon op_main modifiers extract_args \
	s_string s_char s_numeric_u s_numeric_i \
	s_pointer s_logical_b s_logical_o s_logical_xmin s_logical_xmaj	\
	cast

SRC_CHAINED	= ft_lstnew ft_lstdelone ft_lstdel ft_lstadd ft_lstiter ft_lstmap ft_lst_invert_rec \
	ft_lst_push_front ft_lst_push_back ft_lst_pre_alloc ft_lst_merge ft_lst_create_elem ft_lst_invert_it

SRC_SORT	= ft_merge_chain ft_merge_tab

USUALS_DIR	= usuals_tools
PRINTF_DIR	= ft_printf
CHAINED_DIR	= chained_tools
SORT_DIR	= sort_tools

SRC = 	$(addprefix $(USUALS_DIR)/, $(addsuffix .c, $(SRC_USUALS))) \
		$(addprefix $(PRINTF_DIR)/, $(addsuffix .c, $(SRC_PRINTF))) \
		$(addprefix $(CHAINED_DIR)/, $(addsuffix .c, $(SRC_CHAINED))) \
		$(addprefix $(SORT_DIR)/, $(addsuffix .c, $(SRC_SORT)))

TMP = $(basename $(notdir $(SRC)))
OBJ = $(addprefix $(OBJDIR)/, $(addsuffix .o, $(TMP)))

.PHONY: all re clean fclean help

all: $(NAME)

$(NAME): $(OBJ) $(INCDIR)/$(HEADER)
	ar rc $(NAME) $(OBJ)
	ranlib $(NAME)

$(OBJDIR)/%.o: $(SRCDIR)/$(USUALS_DIR)/%.c $(INCDIR)/$(HEADER)
	$(CC) -c $(CFLAGS) $(LIBFT_HEADER)-o $@ $<

$(OBJDIR)/%.o: $(SRCDIR)/$(CHAINED_DIR)/%.c $(INCDIR)/$(HEADER)
	$(CC) -c $(CFLAGS) $(LIBFT_HEADER)-o $@ $<

$(OBJDIR)/%.o: $(SRCDIR)/$(PRINTF_DIR)/%.c $(INCDIR)/$(HEADER)
	$(CC) -c $(CFLAGS) $(LIBFT_HEADER)-o $@ $<

$(OBJDIR)/%.o: $(SRCDIR)/$(SORT_DIR)/%.c $(INCDIR)/$(HEADER)
	$(CC) -c $(CFLAGS) $(LIBFT_HEADER)-o $@ $<

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
