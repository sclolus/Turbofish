# **************************************************************************** #
#                                                                              #
#                                                         :::      ::::::::    #
#    Makefile                                           :+:      :+:    :+:    #
#                                                     +:+ +:+         +:+      #
#    By: vcombey <marvin@42.fr>                     +#+  +:+       +#+         #
#                                                 +#+#+#+#+#+   +#+            #
#    Created: 2016/11/09 16:54:16 by vcombey           #+#    #+#              #
#    Updated: 2017/04/28 10:50:10 by vcombey          ###   ########.fr        #
#                                                                              #
# **************************************************************************** #

PATH              := /toolchain_turbofish/cross/bin/:$(PATH)

INCLUDE = ./

NAME = libft.a
ifeq ($(GNU),yes)
CC = gcc
else
CC = i686-turbofish-gcc 
endif

SRC = ft_dtoa.c \
	ft_abs.c \
	ft_memset.c \
	ft_memmove.c \
	ft_memdel.c \
	ft_memcpy.c \
	ft_memcmp.c \
	ft_memchr.c \
	ft_memccpy.c \
	ft_memalloc.c \
	ft_bzero.c \
	ft_strlcat.c \
	ft_strnew.c \
	ft_atoi.c \
	ft_itoa.c \
	ft_isprint.c \
	ft_isdigit.c \
	ft_isascii.c \
	ft_isblanckspace.c \
	ft_isalpha.c \
	ft_isalnum.c \
	ft_strlen.c \
	ft_strstr.c \
	ft_strequ.c \
	ft_strcmp.c \
	ft_strcpy.c \
	ft_strcat.c \
	ft_strnstr.c \
	ft_strnequ.c \
	ft_strncmp.c \
	ft_strncpy.c \
	ft_strncat.c \
	ft_strdup.c \
	ft_strclr.c \
	ft_strsub.c \
	ft_strdel.c \
	ft_striteri.c \
	ft_striter.c \
	ft_strjoin.c \
	ft_strjoin_free.c \
	ft_strmapi.c \
	ft_strmap.c \
	ft_strchr.c \
	ft_strrchr.c \
	ft_strsplit.c \
	ft_strtrim.c \
	ft_isupper.c \
	ft_tolower.c \
	ft_putstr_fd.c \
	ft_putstr.c \
	ft_putnbr_fd.c \
	ft_putnbr.c \
	ft_putendl_fd.c \
	ft_putendl.c \
	ft_putchar_fd.c \
	ft_putchar.c \
	ft_lstnew.c \
	ft_lstmap.c \
	ft_lstiter.c \
	ft_lstdelone.c \
	ft_lstdel.c \
	ft_lstadd.c \
	ft_islower.c \
	ft_isupper.c \
	get_next_line.c \
	ft_strstrlen.c \
	tab_free.c \
	ft_retmsg.c \
	ft_count_lines.c \
	ft_atoi_safe.c \

SRC_O = $(SRC:.c=.o)

FLAGS = -Wall -Wextra -Werror -g #-fsanitize=address

all: $(NAME)

$(NAME):
	echo $(SRC)
	$(CC) $(FLAGS) -c $(SRC) -I $(INCLUDE)
	@ar rcs $(NAME) $(SRC_O)

clean:
	@/bin/rm -f $(SRC_O)

fclean: clean
	/bin/rm -f $(NAME)

re: fclean all
