/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   sky.c                                              :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/06/08 00:06:10 by bmickael          #+#    #+#             */
/*   Updated: 2017/06/08 00:06:20 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

/*
** void					render_sky(t_env *env, t_rendering_layer *layer);
*/

#include <math.h>
#include <stdlib.h>
#include "wolf3d.h"

void				init_sky(t_env *env, char *file_name)
{
	rendering_layer_init(&(env->scene.sky), file_name);
}

static t_coord_f	calc_tex_coord(t_coord_f angle)
{
	t_coord_f	c_sky;

	c_sky.x = angle.x / (2.f * PI);
	c_sky.y = angle.y / PI;
	if (c_sky.x < 0.f)
		c_sky.x += 1.f;
	if (c_sky.x >= 1.f)
		c_sky.x -= 1.f;
	return (c_sky);
}

static inline float	angle_on_screen(int x)
{
	return (atanf((float)x / (WIDTH / 2)) * (VIEW_ANGLE / 2.f / atanf(1.f)));
}

void				render_sky(t_env *env, t_rendering_layer *layer)
{
	t_coord_i	c;
	t_coord_f	angle;

	layer->n = 0;
	c.y = -1;
	while (++c.y < HEIGHT)
	{
		angle.y = angle_on_screen(HEIGHT / 2 - c.y);
		c.x = -1;
		while (++c.x < WIDTH)
		{
			if (angle.y >= env->scene.columns[c.x].wall_max_angle)
			{
				angle.x = env->scene.columns[c.x].angle_x;
				layer->ij[layer->n] = c;
				layer->uv[layer->n] = calc_tex_coord(angle);
				layer->uv[layer->n].x *= layer->bmp->dim.x - 1;
				layer->uv[layer->n].y *= layer->bmp->dim.y - 1;
				layer->dist[layer->n] = 0.f;
				layer->n++;
			}
		}
	}
	rendering_layer_render(layer);
}
