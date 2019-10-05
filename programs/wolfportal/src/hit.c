/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   hit.c                                              :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: vcombey <vcombey@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/04/26 14:20:43 by vcombey           #+#    #+#             */
/*   Updated: 2017/05/02 11:42:50 by vcombey          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "wolf.h"
#include "math.h"
#include "libft.h"

void	hit_portal_red(double proj, int portal, t_dda dda)
{
	trace_portail(dda.x, (env()->side == 0) ? (dda.side_dist)->x / proj :
			(dda.side_dist)->y / proj, portal, dda);
	if (env()->sideblue != 0)
	{
		env()->wall.x = env()->blue.x;
		env()->wall.y = env()->blue.y;
		transform_direction_ray_portal(portal, dda);
	}
}

void	hit_portal_blue(double proj, int portal, t_dda dda)
{
	trace_portail(dda.x, (env()->side == 0) ? (dda.side_dist)->x / proj :
			(dda.side_dist)->y / proj, portal, dda);
	if (env()->sidered != 0)
	{
		env()->wall.x = env()->red.x;
		env()->wall.y = env()->red.y;
		transform_direction_ray_portal(portal, dda);
	}
}

int		ft_hit(double proj, int portal, t_dda dda)
{
	if (portal == 3 && good_side_portal((dda.step), env()->sidered))
	{
		hit_portal_red(proj, portal, dda);
		return (env()->sideblue == 0);
	}
	else if (portal == 4 && good_side_portal((dda.step), env()->sideblue))
	{
		hit_portal_blue(proj, portal, dda);
		return (env()->sidered == 0);
	}
	else if (portal == 5)
	{
		draw_tourelle(dda.x, (env()->side == 0) ? (dda.side_dist)->x / proj :
				(dda.side_dist)->y / proj);
		return (0);
	}
	return (1);
}
