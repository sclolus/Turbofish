/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   wolf.c                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: vcombey <vcombey@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2016/12/29 17:59:32 by vcombey           #+#    #+#             */
/*   Updated: 2017/05/01 16:30:33 by vcombey          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "wolf.h"
#include "math.h"
#include "libft.h"

void	increment_dist(t_dda dda)
{
	if ((dda.side_dist)->x <= (dda.side_dist)->y)
	{
		(dda.side_dist)->x += (dda.delta_dist)->x;
		env()->wall.x += (dda.step)->x;
		env()->side = 0;
	}
	else
	{
		(dda.side_dist)->y += (dda.delta_dist)->y;
		env()->wall.y += (dda.step)->y;
		env()->side = 1;
	}
}

double	ft_dda(t_dda dda, double proj)
{
	int		portal;

	while (1)
	{
		if ((dda.side_dist)->x < (dda.side_dist)->y && ((portal = env()->map
		[env()->wall.x + (dda.step)->x][env()->wall.y]) > 0))
		{
			env()->side = 0;
			if (ft_hit(proj, portal, dda))
				break ;
		}
		else if ((dda.side_dist)->y < (dda.side_dist)->x && ((portal =
						env()->map[env()->wall.x][env()->wall.y +
						(dda.step)->y]) > 0))
		{
			env()->side = 1;
			if (ft_hit(proj, portal, dda))
				break ;
		}
		increment_dist(dda);
		if ((dda.side_dist)->x > 700 && (dda.side_dist)->y > 700)
			break ;
	}
	return (env()->side == 0) ? (dda.side_dist)->x / proj :
	(dda.side_dist)->y / proj;
}

void	ft_init_dist(t_dda dda)
{
	if ((dda.ray_dir)->x < 0)
	{
		(dda.side_dist)->x = (cam()->pos.x - (int)cam()->pos.x) *
		(dda.delta_dist)->x;
		(dda.step)->x = -1;
	}
	else
	{
		(dda.side_dist)->x = ((int)cam()->pos.x + 1.0 - cam()->pos.x) *
		(dda.delta_dist)->x;
		(dda.step)->x = 1;
	}
	if ((dda.ray_dir)->y < 0)
	{
		(dda.step)->y = -1;
		(dda.side_dist)->y = (cam()->pos.y - (int)cam()->pos.y) *
		(dda.delta_dist)->y;
	}
	else
	{
		(dda.step)->y = 1;
		(dda.side_dist)->y = ((int)cam()->pos.y + 1.0 - cam()->pos.y) *
		(dda.delta_dist)->y;
	}
}

double	ft_calc_dist(double range, t_dda dda)
{
	t_double_pos	side_dist;
	t_double_pos	delta_dist;
	t_int_pos		step;
	double			proj;

	dda.side_dist = &side_dist;
	dda.delta_dist = &delta_dist;
	dda.step = &step;
	(dda.cam_pos)->x = cam()->pos.x;
	(dda.cam_pos)->y = cam()->pos.y;
	env()->wall.x = (int)cam()->pos.x;
	env()->wall.y = (int)cam()->pos.y;
	(dda.ray_dir)->x = cam()->dir.x + range * cam()->plane.x;
	(dda.ray_dir)->y = cam()->dir.y + range * cam()->plane.y;
	env()->ray_dir.x = (dda.ray_dir)->x;
	env()->ray_dir.y = (dda.ray_dir)->y;
	(dda.delta_dist)->x = sqrt(1 + (env()->ray_dir.y * env()->ray_dir.y) /
			(env()->ray_dir.x * env()->ray_dir.x));
	(dda.delta_dist)->y = sqrt(1 + (env()->ray_dir.x * env()->ray_dir.x) /
			(env()->ray_dir.y * env()->ray_dir.y));
	ft_init_dist(dda);
	proj = sqrt((((dda.ray_dir)->y * (dda.ray_dir)->y) +
				((dda.ray_dir)->x * (dda.ray_dir)->x)));
	return (ft_dda(dda, proj));
}

void	ft_wolf(void)
{
	int				x;
	double			dist_wall;
	t_dda			dda;
	t_double_pos	ray_dir;
	t_double_pos	cam_pos;

	dda.cam_pos = &cam_pos;
	dda.ray_dir = &ray_dir;
	x = 0;
	while (x < SCREEN_WIDTH)
	{
		dda.x = x;
		dist_wall = ft_calc_dist(2 * (double)x / (double)SCREEN_WIDTH - 1, dda);
		if (dist_wall < 600)
			ft_trace_colone(x, dist_wall, dda);
		x++;
	}
}
