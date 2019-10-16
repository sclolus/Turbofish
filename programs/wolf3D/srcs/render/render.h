/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   render.h                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/07/05 11:57:45 by bmickael          #+#    #+#             */
/*   Updated: 2017/07/05 11:57:49 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#ifndef RENDER_H
# define RENDER_H
# include "core/wolf3d.h"

typedef struct	s_render
{
	int			n;
	t_env		*e;
}				t_render;

typedef struct	s_sprite_env
{
	t_coord_f	angle_topleft;
	t_coord_f	angle_bottomright;
	t_coord_i	c;
	t_coord_i	c_topleft;
	t_coord_i	c_bottomright;
	t_coord_i	c_max;
	t_coord_f	c_tex;
}				t_sprite_env;

typedef struct	s_wall_finder
{
	t_tile		**tiles;
	float		angle;
	t_coord_f	player;
	t_coord_f	ray_dir;
	t_coord_i	c_i;
	t_coord_f	d_dist;
	t_coord_f	side_dist;
	t_coord_i	step;
}				t_wall_finder;

/*
** render_pix.c
*/

t_pix			get_pix(t_bmp *src, t_coord_f c_src, float dist,
													int bilinear_interpolation);

/*
** find_wall.c
*/

int				find_wall(t_env *env, float angle_x, t_coord_f *intersect,
																float *x_tex);

/*
** render_wall.c
*/

void			init_walls(t_env *e, char **textures, int n);
void			render_wall(t_env *env, t_coord_i c, t_coord_f angle);

/*
** render_floor.c
*/

void			init_floor(t_env *e, char **textures, int n);
void			render_floor(t_env *env, t_coord_i c, t_coord_f angle);

/*
** render_sky.c
*/

void			init_sky(t_env *e, char *file_name);

/*
** render_sprites.c
*/

void			init_sprites(t_env *env, char **textures, int n);
t_sprite		**create_z_buffer_order(t_env *env);
void			render_sprites(t_env *env);
int				m_cmp(void *a, void *b);

/*
** render.c
*/

float			dist(t_coord_f a, t_coord_f b);
void			init_scene(t_env *e);
void			render_scene(t_env *env);
void			scene_to_win(t_env *env);

#endif
