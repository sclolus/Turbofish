/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   trace.c                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: vcombey <vcombey@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2016/12/29 20:35:01 by vcombey           #+#    #+#             */
/*   Updated: 2017/05/02 12:04:33 by vcombey          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "wolf.h"

int			round_neg(double n)
{
	if (n > 0)
		return ((int)n);
	else
		return ((int)n - 1);
}

static void	ft_trace_textur(int x, int draw_start, int draw_end, int texx)
{
	unsigned int	color;
	int				y;

	y = (draw_start < 0) ? 0 : draw_start;
	while (y < SCREEN_HEIGHT && draw_end)
	{
		color = ft_pixelget(texx, (y - draw_start) *
				WALL_P_HEIGHT / (draw_end - draw_start), *texture());
		if (env()->side == 1)
			color = (color >> 1) & 8355711;
		if (ft_pixelget_img(x, y) == 0x0)
			ft_pixelput(x, y, color);
		y++;
	}
}

void		ft_trace_colone(int x, double dist_wall, t_dda dda)
{
	int				lineheight;
	int				draw_start;
	int				draw_end;
	double			wallx;
	int				texx;

	lineheight = (int)((SCREEN_HEIGHT / dist_wall) * 1.5);
	draw_start = -lineheight / 2 + SCREEN_HEIGHT / 2;
	draw_end = lineheight / 2 + SCREEN_HEIGHT / 2;
	if (env()->side == 0)
		wallx = (dda.cam_pos)->y + dist_wall * (dda.ray_dir)->y;
	else
		wallx = (dda.cam_pos)->x + dist_wall * (dda.ray_dir)->x;
	wallx -= round_neg(wallx);
	texx = (int)(wallx * (double)WALL_P_WIDTH);
	if (env()->side == 0 && (dda.ray_dir)->x > 0)
		texx = WALL_P_WIDTH - texx - 1;
	if (env()->side == 1 && (dda.ray_dir)->y < 0)
		texx = WALL_P_WIDTH - texx - 1;
	ft_trace_textur(x, draw_start, draw_end, texx);
}
