/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   trace_portal.c                                     :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: vcombey <marvin@42.fr>                     +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/04/26 22:47:55 by vcombey           #+#    #+#             */
/*   Updated: 2017/05/02 11:36:34 by vcombey          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "wolf.h"

void	trace_portal_blue_step_y(t_interval w, int x, int y, int texx)
{
	int		color;

	if ((color = ft_pixelget(texx, (y - w.start) * portal_blue()->height /
					(w.end - w.start), *portal_blue())) == 0x00ff00)
	{
		color = ft_pixelget(texx, (y - w.start) * WALL_P_HEIGHT /
				(w.end - w.start), *texture());
		if (env()->side == 1)
			color = (color >> 1) & 8355711;
	}
	if (color == 0 && env()->sidered == 0)
		color = 0x00B2DE;
	if (ft_pixelget_img(x, y) == 0x0)
		ft_pixelput(x, y, color);
}

void	trace_portal_red_step_y(t_interval w, int x, int y, int texx)
{
	int		color;

	if ((color = ft_pixelget(texx, (y - w.start) * portal_blue()->height /
					(w.end - w.start), *portal_red())) == 0x00ff00)
	{
		color = ft_pixelget(texx, (y - w.start) * WALL_P_HEIGHT /
				(w.end - w.start), *texture());
		if (env()->side == 1)
			color = (color >> 1) & 8355711;
	}
	if (color == 0 && env()->sideblue == 0)
		color = 0xFFA500;
	if (ft_pixelget_img(x, y) == 0x0)
		ft_pixelput(x, y, color);
}

void	trace_portal_loop(int portal, int x, t_interval w, int texx)
{
	int		y;

	y = (w.start < 0) ? 0 : w.start;
	while (y < SCREEN_HEIGHT && y < w.end)
	{
		if (portal == 3)
			trace_portal_red_step_y(w, x, y, texx);
		else
			trace_portal_blue_step_y(w, x, y, texx);
		y++;
	}
}

void	trace_portail(int x, double dist_wall, int portal, t_dda dda)
{
	int			lineheight;
	t_interval	w;
	double		wallx;
	int			texx;

	lineheight = (int)((SCREEN_HEIGHT / dist_wall) * 1.5);
	w.start = -lineheight / 2 + SCREEN_HEIGHT / 2;
	w.end = lineheight / 2 + SCREEN_HEIGHT / 2;
	if (env()->side == 0)
		wallx = (dda.cam_pos)->y + dist_wall * (dda.ray_dir)->y;
	else
		wallx = (dda.cam_pos)->x + dist_wall * (dda.ray_dir)->x;
	wallx -= round_neg(wallx);
	texx = (int)(wallx * (double)portal_blue()->width);
	if (env()->side == 0 && (dda.ray_dir)->x > 0)
		texx = portal_blue()->width - texx - 1;
	if (env()->side == 1 && (dda.ray_dir)->y < 0)
		texx = portal_blue()->width - texx - 1;
	trace_portal_loop(portal, x, w, texx);
}
