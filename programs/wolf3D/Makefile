NAME = wolf3d

### MAIN FLAGS ###

ifeq ($(GNU),yes)
CC = gcc
UNAME_S := $(shell uname -s)
ifeq ($(UNAME_S),Linux)
	_MLX = minilibx_linux
	ifeq ($(DEBUG),yes)
		CFLAGS = -D GNU -Wall -Wextra -std=c99 -g -O0 -fsanitize=address -I $(INCDIR) -I./$(MINILIBX) -DLINUX
	else
		CFLAGS = -D GNU -Ofast -march=native -fomit-frame-pointer -Wall -Wextra -std=c99 -I $(INCDIR) -I./$(MINILIBX) -DLINUX
	endif
endif
ifeq ($(UNAME_S),Darwin)
	_MLX = minilibx_elcapitan
	ifeq ($(DEBUG),yes)
		CFLAGS = -D GNU -Wall -Wextra -std=c99 -g -O0 -fsanitize=address
	else
		CFLAGS = -D GNU -Ofast -fomit-frame-pointer -Wall -Wextra -std=c99
	endif
endif
else
# export TURBOFISH_ROOT in you shell configuration file to execute this Makefile independently
include $(TURBOFISH_ROOT)/boilerplates.mk
CC = i686-turbofish-gcc
CFLAGS = -Wall -Wextra -fno-omit-frame-pointer
endif

### SOURCES ###

SRC_COMMON = common
SRC_CORE = wolf3d image_mlx_tools init_mlx actions keyboard load_config \
			debug timer get_wall_infos define_mouvements move_sprites
SRC_RENDER = render_pix find_wall render_wall render_floor render_sky render_sprites render misc
SRC_PARSE = constructor load_map get_next_line get_player_location get_sprites get_map_struct verif_texture_range
SRC_OVERLAY = draw_line draw minimap
SRC_BMP = bmp_load bmp_save

SRC_LIST = $(SRC_COMMON) $(SRC_CORE) $(SRC_PARSE) $(SRC_BMP) $(SRC_OVERLAY) $(SRC_RENDER)
VPATH = srcs srcs/core srcs/parse srcs/bmp srcs/overlay srcs/render

## HEADERS

HEADERS = common.h wolf3d.h parse.h internal_parse.h bmp.h internal_bmp.h overlay.h internal_overlay.h render.h

### LIBRAIRIES ###

LIB_DIR = libs

ifeq ($(GNU),yes)
MLX = $(addprefix $(LIB_DIR)/, $(_MLX))
endif

### ~~~~~~~~~~ ###

SRC = $(addsuffix .c, $(SRC_LIST))
OBJ_DIR = objs
TMP = $(basename $(notdir $(SRC)))
OBJ = $(addprefix $(OBJ_DIR)/, $(addsuffix .o, $(TMP)))

ifeq ($(GNU),yes)
IFLAGS = -Isrcs -I$(MLX)
ifeq ($(UNAME_S),Linux)
	LDFLAGS = -L $(MLX) -lmlx -L/usr/include/../lib -lXext -lX11 -lm -lbsd -lpthread
endif
ifeq ($(UNAME_S),Darwin)
	LDFLAGS = -framework openGL -framework AppKit $(MLX)/libmlx.a
endif
else
IFLAGS = -Isrcs
endif

.PHONY: all clean fclean re help

all: $(NAME)

$(NAME): $(OBJ)
ifeq ($(GNU),yes)
	make -C $(MLX)/ all
endif
	$(CC) $(CFLAGS) -o $(NAME) $(OBJ) $(LDFLAGS)

$(OBJ_DIR)/%.o: %.c $(HEADERS)
	$(CC) -c $(CFLAGS) -o $@ $< $(IFLAGS)

clean:
ifeq ($(GNU),yes)
	make -C $(MLX)/ clean
endif
	rm -f $(OBJ)

fclean: clean
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

exec:
	./wolf3d maps/mapZ.map
