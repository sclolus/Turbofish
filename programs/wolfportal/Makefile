# export TURBOFISH_ROOT in you shell configuration file to execute this Makefile independently
include $(TURBOFISH_ROOT)/boilerplates.mk
# **************************************************************************** #
#                                                                              #
#                                                         :::      ::::::::    #
#    Makefile                                           :+:      :+:    :+:    #
#                                                     +:+ +:+         +:+      #
#    By: vcombey <marvin@42.fr>                     +#+  +:+       +#+         #
#                                                 +#+#+#+#+#+   +#+            #
#    Created: 2016/12/30 19:59:01 by vcombey           #+#    #+#              #
#    Updated: 2017/05/01 18:11:47 by vcombey          ###   ########.fr        #
#                                                                              #
# **************************************************************************** #

PATH              := /toolchain_turbofish/cross/bin/:$(PATH)
# CC = gcc 
CC = i686-turbofish-gcc 

NAME =	wolf3d

SRC =	main.c			\
		parsing.c		\
		ft_pixelput.c	\
		ft_pixelget.c	\
		ft_retmsg.c		\
		env.c			\
		map.c			\
		camera.c		\
		wolf.c			\
		trace.c			\
		key_event.c		\
		portal_gun.c	\
		teleportation.c	\
		cross.c			\
		ft_rot_pos.c	\
		hit_portal.c	\
		hit.c			\
		draw_portal_gun.c	\
		struct.c		\
		key_pressed.c	\
		init_texture.c	\
		trace_portal.c	\
		tourelle.c		\
		trace_line.c	\
		floor_casting.c	\
		mlx.c	\

INCLUDE = -I ./libft -I ./include -I ./minilibx_macos/


OBJS = $(addprefix objs/, $(SRC:.c=.o))

CFLAGS += -g -o2 -flto
all: $(NAME)

$(NAME): dis $(OBJS)
	make -C ./libft/
	# $(CC) -g $(OBJS) -L minilibx_linux -lmlx -lm -lbsd -lXext -lX11 -L libft -lft -o2 -flto -o $(NAME)
	$(CC) -g $(OBJS) -L libft -lft -o2 -flto -o $(NAME)
	sudo cp -v $(NAME) $(SYSTEM_ROOT)/

clean:
	make clean -C ./libft/
	/bin/rm -rf objs

fclean: clean
	/bin/rm -f ./libft/libft.a
	/bin/rm -f $(NAME)

re: fclean all

objs/%.o : src/%.c
	@/bin/mkdir -p objs
	echo "compiling" $<
	$(CC) $(INCLUDE) $(CFLAGS) -c -o $@ $<

dis:
	@echo " __   __  ___     ______    ___       _______   _______  ________   "
	@echo "|\"  |/  \|  \"|   /    \" \  |\"  |     /\"     \"| /\" __   )|\"      \"\  "
	@echo "|'  /    \:  |  // ____  \ ||  |    (: ______)(__/ _) ./(.  ___  :) "
	@echo "|: /'        | /  /    ) :)|:  |     \/    |      /  // |: \   ) || "
	@echo " \//  /\'    |(: (____/ //  \  |___  // ___)   __ \_ \\\\\\\ (| (___\ || "
	@echo " /   /  \\\    | \        /  ( \_|:  \(:  (     (: \__) :\|:       :) "
	@echo "|___/    \___|  \\\"_____/    \_______)\__/      \_______)(________/ "
                                                                                                                            
.PHONY: all clean fclean re
