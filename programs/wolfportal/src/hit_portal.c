/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   hit_portal.c                                       :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: vcombey <vcombey@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/04/25 21:26:52 by vcombey           #+#    #+#             */
/*   Updated: 2017/05/02 11:18:54 by vcombey          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "wolf.h"
#include "math.h"
#include "libft.h"

int		good_side_portal(t_int_pos *step, int side_color)
{
	if (env()->side == 0 && ft_abs(side_color) == 1 && side_color == step->x)
		return (1);
	else if (env()->side == 1 && ft_abs(side_color) == 2 && side_color ==
			2 * step->y)
		return (1);
	return (0);
}

int		transform_sidecolor(int side_color)
{
	if (side_color < 0)
		return ((side_color == -1) ? 3 : 4);
	return (side_color);
}

double	ft_min_double(double a, double b)
{
	return (a < b) ? a : b;
}

void	transform_direction_ray_portal_perpendicular(int portal, t_dda dda)
{
	int		a;
	int		b;

	ft_swap_double_pos(dda.delta_dist);
	a = (portal == 3) ? transform_sidecolor(env()->sidered) :
		transform_sidecolor(env()->sideblue);
	b = (portal == 3) ? transform_sidecolor(env()->sideblue) :
		transform_sidecolor(env()->sidered);
	if ((a < b && !(a == 1 && b == 4)) || (a == 4 && b == 1))
	{
		ft_rev_rot_int(dda.step);
		ft_rev_rot_double(dda.ray_dir);
		ft_rev_rot_double(dda.cam_pos);
	}
	else if (a > b || (a == 1 && b == 4))
	{
		ft_rot_int(dda.step);
		ft_rot_double(dda.ray_dir);
		ft_rot_double(dda.cam_pos);
	}
	ft_swap_double_pos(dda.side_dist);
}

void	transform_direction_ray_portal(int portal, t_dda dda)
{
	if (env()->sideblue == env()->sidered)
	{
		(dda.step)->x *= -1;
		(dda.step)->y *= -1;
	}
	else if (ft_abs(env()->sideblue) != ft_abs(env()->sidered))
		transform_direction_ray_portal_perpendicular(portal, dda);
}
