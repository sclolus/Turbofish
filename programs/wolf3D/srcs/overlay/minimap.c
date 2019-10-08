/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   minimap.c                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/06/06 01:30:50 by bmickael          #+#    #+#             */
/*   Updated: 2017/06/06 01:30:52 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include <sys/time.h>
#include <stdio.h>
#include <math.h>
#include "core/wolf3d.h"
#include "overlay/overlay.h"
#include "overlay/internal_overlay.h"

static float	is_close(float min, float max, float a, float b)
{
	float v;
	float u;

	v = (a > b) ? a - b : b - a;
	u = (max - min) - v;
	u = (v < u) ? v : u;
	return ((u < 1) ? (1 - (u / 1)) : 0);
}

static void		locate_enemy(t_env *e, float ref_angle)
{
	t_coord_f	l;
	int			i;
	t_pix		pix;
	float		dist;
	float		angle;

	pix.i = 0x800000;
	i = -1;
	while (++i < e->n_sprites)
	{
		l.x = e->sprites[i].location.x - e->player.location.x;
		l.y = e->sprites[i].location.y - e->player.location.y;
		if ((dist = sqrtf((l.x * l.x) + (l.y * l.y))) < (MAP_DEPTH - 1))
		{
			angle = atan2f((l.y), (l.x));
			pix.i = 0xFF * is_close(-PI, PI, angle, ref_angle);
			if (!pix.i)
				continue ;
			pix.i <<= 8;
			l.x = l.x * (MAP_RADIUS / MAP_DEPTH) + MAP_ORIGIN_X;
			l.y = l.y * (MAP_RADIUS / MAP_DEPTH) + MAP_ORIGIN_Y;
			draw_circle(e->scene.scene,
									(t_coord_i){(int)l.x, (int)l.y}, 4, pix);
		}
	}
}

void			draw_minimap(t_env *e)
{
	struct timeval	spec;
	t_coord_i		l1;
	t_pix			color;
	float			angle;
	t_line			line;

	l1 = (t_coord_i){MAP_ORIGIN_X, MAP_ORIGIN_Y};
	draw_arrow(e->scene.scene, l1, e->player.angle);
	color.i = 0xffffff;
	draw_circle(e->scene.scene, l1, MAP_RADIUS, color);
	gettimeofday(&spec, NULL);
	angle = (float)((((((int)spec.tv_sec & 0x0F) * 1000) +
		round(spec.tv_usec / 1.0e3)) / 2000) * PI * 2);
	line.p1 = l1;
	line.p2.x = (floor)(cosf(angle) * MAP_RADIUS) + MAP_ORIGIN_X;
	line.p2.y = (floor)(sinf(angle) * MAP_RADIUS) + MAP_ORIGIN_Y;
	line.b_pix.i = 0xFFFF00;
	line.f_pix.i = 0x00FFFF;
	draw_line(e->scene.scene, &line);
	locate_enemy(e, atan2f(sinf(angle), cosf(angle)));
}
