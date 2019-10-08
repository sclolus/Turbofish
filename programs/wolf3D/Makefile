NAME = wolf3d
CC = gcc

### MAIN FLAGS ###

UNAME_S := $(shell uname -s)
ifeq ($(UNAME_S),Linux)
	_MLX = minilibx_linux
	ifeq ($(DEBUG),yes)
		CFLAGS = -Wall -Wextra -std=c99 -g -O0 -fsanitize=address -I $(INCDIR) -I $(LIBFT_HEADER) -I./$(MINILIBX) -DLINUX
	else
		CFLAGS = -Ofast -march=native -fomit-frame-pointer -Wall -Wextra -std=c99 -I $(INCDIR) -I $(LIBFT_HEADER) -I./$(MINILIBX) -DLINUX
	endif
endif
ifeq ($(UNAME_S),Darwin)
	_MLX = minilibx_elcapitan
	ifeq ($(DEBUG),yes)
		CFLAGS = -Wall -Wextra -std=c99 -g -O0 -fsanitize=address
	else
		CFLAGS = -Ofast -fomit-frame-pointer -Wall -Wextra -std=c99
	endif
endif

### SOURCES ###

SRC_CORE = wolf3d image_mlx_tools init_mlx actions keyboard load_config \
			debug timer get_wall_infos define_mouvements move_sprites
SRC_RENDER = render_pix find_wall render_wall render_floor render_sky render_sprites render misc
SRC_PARSE = constructor load_map get_next_line get_player_location get_sprites get_map_struct verif_texture_range
SRC_OVERLAY = draw_line draw minimap
SRC_BMP = bmp_load bmp_save


SRC_LIST = $(SRC_CORE) $(SRC_PARSE) $(SRC_BMP) $(SRC_OVERLAY) $(SRC_RENDER)
VPATH = srcs/core srcs/parse srcs/bmp srcs/overlay srcs/render

## HEADERS

HEADERS = wolf3d.h parse.h internal_parse.h bmp.h internal_bmp.h overlay.h internal_overlay.h render.h

### LIBRAIRIES ###

LIB_DIR = libs
MLX = $(addprefix $(LIB_DIR)/, $(_MLX))
_LIBFT = libft
LIBFT = $(addprefix $(LIB_DIR)/, $(_LIBFT))

### ~~~~~~~~~~ ###

SRC = $(addsuffix .c, $(SRC_LIST))
OBJ_DIR = objs
TMP = $(basename $(notdir $(SRC)))
OBJ = $(addprefix $(OBJ_DIR)/, $(addsuffix .o, $(TMP)))


IFLAGS = -Isrcs -I$(LIBFT)/includes -I$(MLX)
ifeq ($(UNAME_S),Linux)
	LDFLAGS = -L$(LIBFT) -lft -L $(MLX) -lmlx -L/usr/include/../lib -lXext -lX11 -lm -lbsd -lpthread
endif
ifeq ($(UNAME_S),Darwin)
	LDFLAGS = -L$(LIBFT) -lft -framework openGL -framework AppKit $(MLX)/libmlx.a
endif

.PHONY: all clean fclean re help

all: $(NAME)

$(NAME): $(OBJ)
	make -C $(MLX)/ all
	make -C $(LIBFT)/ all DEBUG=$(DEBUG)
	$(CC) $(CFLAGS) -o $(NAME) $(OBJ) $(LDFLAGS)

$(OBJ_DIR)/%.o: %.c $(HEADERS)
	$(CC) -c $(CFLAGS) -o $@ $< $(IFLAGS)

clean:
	make -C $(MLX)/ clean
	make -C $(LIBFT)/ clean
	rm -f $(OBJ)

fclean: clean
	make -C $(LIBFT)/ fclean
	rm -f $(NAME)

re: fclean all

help:
	@echo
	@echo "Programm $(NAME)"
	@echo
	@echo "--------------------------------------------------------------------------"
	@echo " Disp rules."
	@echo
	@echo " all     : Compile the program $(NAME) into $(BINDIR) directory."
	@echo " re      : Recompile all objets of the programm."
	@echo " clean   : Remove objects."
	@echo " fclean  : Remove objects and programm."
	@echo " help    : Display this."
	@echo "--------------------------------------------------------------------------"
