/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   draw.c                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/06/06 03:36:26 by bmickael          #+#    #+#             */
/*   Updated: 2017/06/06 03:36:28 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include <stdlib.h>
#include <math.h>
#include "core/wolf3d.h"
#include "overlay/internal_overlay.h"

static inline void	plot_pixel(t_pix *scene, t_coord_i l, t_pix pix)
{
	int offset;

	offset = (l.y * WIDTH) + l.x;
	if (offset >= (WIDTH * HEIGHT) || offset < 0)
		return ;
	scene[offset] = pix;
}

void				draw_arrow(t_pix *scene, t_coord_i c, float angle)
{
	t_line		line;
	t_coord_i	l1;
	t_coord_i	l2;
	t_coord_i	l3;

	l1.y = c.y + (int)(((sinf(angle + PI * 3 / 4)) * ARROW_RADIUS));
	l1.x = c.x + (int)(((cosf(angle + PI * 3 / 4)) * ARROW_RADIUS));
	l2.y = c.y + (int)(((sinf(angle - PI * 3 / 4)) * ARROW_RADIUS));
	l2.x = c.x + (int)(((cosf(angle - PI * 3 / 4)) * ARROW_RADIUS));
	l3.y = c.y + (int)(((sinf(angle)) * LONG_ARROW_RADIUS));
	l3.x = c.x + (int)(((cosf(angle)) * LONG_ARROW_RADIUS));
	line.p1 = l1;
	line.p2 = l2;
	line.b_pix.i = 0x0000FF;
	line.f_pix.i = 0x00FF00;
	draw_line(scene, &line);
	line.p1 = l2;
	line.p2 = l3;
	line.b_pix.i = 0x00FF00;
	line.f_pix.i = 0xFF0000;
	draw_line(scene, &line);
	line.p1 = l1;
	line.b_pix.i = 0x0000FF;
	draw_line(scene, &line);
}

void				draw_box(t_coord_i p1, t_coord_i p2, t_pix pix,
																t_pix *scene)
{
	t_line line;

	line.b_pix = pix;
	line.f_pix = pix;
	line.p1 = p1;
	line.p2.y = p1.y;
	line.p2.x = p2.x;
	draw_line(scene, &line);
	line.p1 = p2;
	draw_line(scene, &line);
	line.p2.x = p1.x;
	line.p2.y = p2.y;
	draw_line(scene, &line);
	line.p1 = p1;
	draw_line(scene, &line);
}

void				fill_box(t_coord_i p1, t_coord_i p2, t_pix pix,
																t_pix *scene)
{
	t_line	line;
	int		i;

	line.b_pix = pix;
	line.f_pix = pix;
	line.p1.x = p1.x;
	line.p2.x = p2.x;
	i = p1.y;
	while (i <= p2.y)
	{
		line.p1.y = i;
		line.p2.y = i;
		draw_line(scene, &line);
		i++;
	}
}

/*
** Les equations en dessous decoulent de la formule generale d'un cercle:
** x² + y² = r²
*/

void				draw_circle(t_pix *scene, t_coord_i position, int radius,
																	t_pix color)
{
	t_coord_i	location;
	float		x;
	float		y;

	x = -radius - 1;
	while (++x <= radius)
	{
		y = sqrt((radius * radius) - (x * x));
		location.x = x + position.x;
		location.y = floor(y) + position.y;
		plot_pixel(scene, location, color);
		location.y = -floor(y) + position.y;
		plot_pixel(scene, location, color);
	}
	y = -radius - 1;
	while (++y <= radius)
	{
		x = sqrt((radius * radius) - (y * y));
		location.y = y + position.y;
		location.x = floor(x) + position.x;
		plot_pixel(scene, location, color);
		location.x = -floor(x) + position.x;
		plot_pixel(scene, location, color);
	}
}
