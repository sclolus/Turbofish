/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   wolf3d.h                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: stoupin <stoupin@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/06/10 01:46:12 by bmickael          #+#    #+#             */
/*   Updated: 2018/02/01 17:16:52 by stoupin          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#ifndef WOLF3D_H
# define WOLF3D_H

# include <math.h>

# include "libft.h"
# include "graphic_types.h"
# include "parse/internal_parse.h"

#ifdef GNU
# include "mlx.h"
#endif

# define DEBUG_KEYBOARD		FALSE

# define PI					3.14159265358979323846

# define NOSTALGIA_FACTOR	1
# define WIDTH				(1920 / NOSTALGIA_FACTOR)
# define HEIGHT				(1080 / NOSTALGIA_FACTOR)
# define SCREENSIZE			(WIDTH * HEIGHT)

# define RATIO				4
# define VIEW_ANGLE			(2.f * PI / RATIO)
# define SHADOW_LIMIT		7

# define N_FLOOR_BMP		3
# define N_WALL_BMP			4
# define N_SPRITE_BMP		3

# define MAP_ORIGIN_X			(WIDTH - (120 >> (NOSTALGIA_FACTOR >> 1)))
# define MAP_ORIGIN_Y			(120 >> (NOSTALGIA_FACTOR >> 1))
# define MAP_RADIUS				(100 >> (NOSTALGIA_FACTOR >> 1))
# define MAP_DEPTH				20
# define LONG_ARROW_RADIUS		(30 >> (NOSTALGIA_FACTOR >> 1))
# define ARROW_RADIUS			(15 >> (NOSTALGIA_FACTOR >> 1))

/*
** maximum size of the map files in bytes
*/

# define MAX_MAP_SIZE			(4*1024*1024)

# define KEYRELEASEMASK			0xFF
# define KEYPRESSMASK			0xFF

# define X11_KEY_RELEASE		3
# define X11_KEY_PRESS			2
# define X11_DESTROY_NOTIFY		17
# define X11_BUTTON_4			4
# define X11_MOTION_NOTIFY		6

# define KEYB_1				18
# define KEYB_2				19
# define KEYB_3				20
# define KEYB_4				21
# define KEYB_5				23
# define KEYB_6				22
# define KEYB_7				26
# define KEYB_8				28
# define KEYB_9				26
# define KEYB_C				8
# define KEYB_P				35
# define KEYB_R				15
# define KEYB_PLUS			69
# define KEYB_MINUS			78
# define KEYB_MMO_W			13
# define KEYB_MMO_S			1
# define KEYB_MMO_A			0
# define KEYB_MMO_D			2

# ifdef LINUX

/*
** Linux scanCodes
*/

#  define KEYB_M			109
#  define KEYB_HELP			44
#  define KEYB_ESCAPE		27
#  define KEYB_ARROW_UP		82
#  define KEYB_ARROW_DOWN	84
#  define KEYB_ARROW_LEFT	81
#  define KEYB_ARROW_RIGHT	83

# else

/*
** OSX scanCodes
*/

#  define KEYB_M			46
#  define KEYB_HELP			44
#  define KEYB_ESCAPE		53
#  define KEYB_ARROW_UP		126
#  define KEYB_ARROW_DOWN	125
#  define KEYB_ARROW_LEFT	123
#  define KEYB_ARROW_RIGHT	124

# endif

# define N_CONTROL			4

typedef struct			s_bmp
{
	t_coord_i			dim;
	t_pix				*pix;
}						t_bmp;

typedef struct			s_player
{
	t_coord_f			location;
	float				angle;
	float				height;
	int					lives;
}						t_player;

/*
** size_x -> longueur de la carte.
** size_y -> hauteur de la carte.
** scale -> echelle pour les petites cartes, varie de 1 a 10.
** Par convention, une map ne peut etre plus petite que 1 * 1 !
** au mieux, 10 * 10 cases sont affichees.
** le joueur est centre au millieu sauf s'il va vers un bord.
** la carte est centree.
*/

typedef struct			s_map
{
	t_coord_i			size;
	float				scale;
}						t_map;

typedef struct			s_weapon
{
	int					i;
	int					j;
	t_bmp				*data;
	t_bmp				*data_2;
}						t_weapon;

typedef struct s_env	t_env;

typedef struct			s_tile
{
	unsigned int		value;
}						t_tile;

typedef struct			s_column
{
	float				wall_h_dist;
	float				wall_x_tex;
	float				wall_min_angle;
	float				wall_max_angle;
	int					type;
}						t_column;

typedef struct			s_rendering_layer
{
	t_coord_i			ij;
	t_coord_f			uv;
	int					type;
	float				dist;
	t_pix				result;
}						t_rendering_layer;

typedef struct			s_scene
{
	t_bmp				*bmp_wall;
	t_bmp				*bmp_floor;
	t_bmp				*bmp_sprite;
	int					n_layer_wall;
	int					n_layer_floor;
	int					n_layer_sprite;
	t_rendering_layer	*wall;
	t_rendering_layer	*floor;
	t_rendering_layer	*sprites;
	t_column			*columns;
	t_pix				*scene;
}						t_scene;

/*
** AI rudimentaire Roaming
** last_time -> moment ou a ete prise la derniere decision.
** goal -> coordonnes ou amenent la derniere decision.
*/

typedef struct			s_sprite
{
	int					type;
	t_coord_f			location;
	float				dist;
	float				angle0_x;

	long int			last_time;
	t_coord_f			origin;
	t_coord_f			goal;
}						t_sprite;

typedef struct			s_sky
{
	int					pos;
	t_bmp				*data;
}						t_sky;

typedef struct			s_vector_2
{
	float				dx;
	float				dy;
	float				module;
}						t_vector_2;

typedef struct			s_wall_vector
{
	t_vector_2			v;
	t_coord_f			norm;
}						t_wall_vector;

typedef struct			s_wall_info
{
	t_coord_f			ray_pos;
	t_coord_f			ray_dir;
	t_coord_i			map;
	t_coord_i			step;
	t_coord_f			delta_dist;
	t_coord_f			side_dist;
	t_wall_vector		w;
}						t_wall_info;

struct					s_env
{
	void				*mlx;
	void				*win;
	void				*image;
	int					bpp;
	int					endian;
	int					s_l;
	int					display_minimap;
	t_player			player;
	t_map				map;
	t_weapon			weapon;
	t_sky				*sky;
	t_pix				*img_string;
	unsigned long int	keyb[256];
	float				wall_height;
	float				sprite_height;
	t_tile				**map_tiles;
	float				angle_x[WIDTH];
	float				angle_y[HEIGHT];
	float				dist_floor[HEIGHT];
	float				atan_list[HEIGHT];
	float				cos_list[WIDTH];
	t_scene				scene;
	int					inter_state;
	unsigned long int	inter_time;
	int					n_sprites;
	t_sprite			*sprites;
	t_map_content		*content;
};

typedef struct			s_modify_coord
{
	int					keycode_1;
	int					keycode_2;
	float				q;
	float				l;
}						t_modify_coord;

# define NB_CORES		8

typedef struct			s_thread_format
{
	int					n;
	int					inter;
	t_bmp				*bmp;
	t_rendering_layer	*layer;
}						t_th_format;

typedef struct			s_thread_put
{
	int					n;
	t_rendering_layer	*layer;
	t_pix				*pix;

}						t_thread_put;

t_pix					get_pix_sp(t_bmp *src, t_coord_f c_src);

t_bmp					*load_bitmap(char **name, int n);

int						init_mlx(t_env *env, char *window_name, int width,
																	int height);
int						create_mlx_image(t_env *e);
void					set_mlx_image_bg_color(t_env *e, t_pix color);
int						exit_mlx(t_env *e);

int						mlx_key_release(int keycode, t_env *e);
int						mlx_key_press(int keycode, t_env *e);
int						common_action(t_env *e);
void					interpolate_switch(t_env *e, unsigned long int m);

void					view_map(t_tile **map, int width, int height);
void					eval_fps(t_env *e);

int						move_player(t_env *e);

void					draw_minimap(t_env *e);

unsigned long int		get_time(void);

int						err_usage(char *cmd);
int						err_msg(char *msg);

void					*thread_x_sprite(void *arg);
void					*thread_x_base(void *arg);

t_wall_vector			get_wall_info(t_tile **tiles, float angle,
														t_coord_f location);
float					mvt_right(t_tile **map, t_coord_f mvt,
														t_coord_f location);
float					mvt_left(t_tile **map, t_coord_f mvt,
														t_coord_f location);
float					mvt_top(t_tile **map, t_coord_f mvt,
														t_coord_f location);
float					mvt_back(t_tile **map, t_coord_f mvt,
														t_coord_f location);
void					init_sprite_ai(t_env *e);
void					animate_sprites(t_env *e);
#endif
