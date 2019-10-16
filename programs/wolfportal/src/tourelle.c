/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   tourelle.c                                         :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: vcombey <vcombey@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/04/27 13:53:26 by vcombey           #+#    #+#             */
/*   Updated: 2017/05/01 10:32:49 by vcombey          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "wolf.h"
#include "libft.h"
#include <stdlib.h>

void		laser(int x, int y)
{
	t_double_pos	a;
	t_double_pos	b;

	a.y = (double)x;
	a.x = (double)y;
	b.y = (double)((double)x - (env()->ray_dir).y * 1000);
	b.x = (double)((double)y + ft_double_abs((env()->ray_dir).x) * 1000);
	ft_trace_line(a, b);
}

void		trace_tourelle_step_y(int tourelle_y, int x, int y, double wallx)
{
	unsigned int		color;

	if ((int)wallx == 161 && tourelle_y == (185 - 85))
		laser(x, y);
	if (((color = ft_pixelget((int)wallx, tourelle_y, *tourelle())) !=
				0xFF000000) && (ft_pixelget_img(x, y) == 0x0))
		ft_pixelput(x, y, color);
}

void		draw_tourelle(int x, double dist_wall)
{
	int		lineheight;
	int		draw_start;
	int		draw_end;
	double	wallx;
	int		y;

	lineheight = (int)((SCREEN_HEIGHT / (dist_wall)) * 1.5);
	draw_start = SCREEN_HEIGHT / 2;
	draw_end = lineheight / 2 + SCREEN_HEIGHT / 2;
	if (env()->side == 0)
		return ;
	wallx = cam()->pos.x + dist_wall * env()->ray_dir.x;
	wallx -= (int)wallx;
	wallx = (wallx * (double)tourelle()->width);
	if (env()->side == 0 && env()->ray_dir.x > 0)
		wallx = tourelle()->width - wallx - 1;
	if (env()->side == 1 && env()->ray_dir.y < 0)
		wallx = tourelle()->width - wallx - 1;
	y = (draw_start < 0) ? 0 : draw_start;
	while (y < SCREEN_HEIGHT && y < draw_end)
	{
		trace_tourelle_step_y((y - draw_start) * tourelle()->height /
				(draw_end - draw_start), x, y, wallx);
		y++;
	}
}

t_texture	*tourelle(void)
{
	static t_texture	t;

	return (&t);
}

void		tourelle_shoot(void)
{
	int		x;
	int		y;

	env()->life--;
	y = 0;
	while (y < SCREEN_HEIGHT)
	{
		x = 0;
		while (x < SCREEN_WIDTH)
		{
			ft_pixelput(x, y, 0xF0FF0000);
			x++;
		}
		y++;
	}
	if (env()->life <= 0)
	{
		/* system("killall afplay"); */
		ft_putstr2("GAME OVER\n");
		exit(0);
	}
}
