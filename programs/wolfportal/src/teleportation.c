/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   teleportation.c                                    :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: vcombey <marvin@42.fr>                     +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/04/19 10:19:26 by vcombey           #+#    #+#             */
/*   Updated: 2017/05/02 11:47:17 by vcombey          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "wolf.h"
#include "libft.h"

void	teleport_rot_perpendicular(int portal)
{
	int		a;
	int		b;

	if (portal == 3)
	{
		a = transform_sidecolor(env()->sidered);
		b = transform_sidecolor(env()->sideblue);
	}
	else
	{
		b = transform_sidecolor(env()->sidered);
		a = transform_sidecolor(env()->sideblue);
	}
	if ((a < b && !(a == 1 && b == 4)) || (a == 4 && b == 1))
	{
		ft_rev_rot_double(&cam()->dir);
		ft_rev_rot_double(&cam()->plane);
	}
	else if (a > b || (a == 1 && b == 4))
	{
		ft_rot_double(&cam()->dir);
		ft_rot_double(&cam()->plane);
	}
}

void	teleport_rot(int portal)
{
	if (env()->sideblue == env()->sidered)
	{
		cam()->dir.x *= -1;
		cam()->dir.y *= -1;
		cam()->plane.x *= -1;
		cam()->plane.y *= -1;
	}
	else if (ft_abs(env()->sideblue) != ft_abs(env()->sidered))
		teleport_rot_perpendicular(portal);
}

void	teleport_pos_blue(int portal)
{
	cam()->pos.x = (double)env()->blue.x;
	cam()->pos.y = (double)env()->blue.y;
	if (ft_abs(env()->sideblue) == 1)
	{
		cam()->pos.x += (env()->sideblue == -1) ? 1.0 : 0;
		cam()->pos.y += 0.5;
	}
	else
	{
		cam()->pos.y += (env()->sideblue == -2) ? 1.0 : 0;
		cam()->pos.x += 0.5;
	}
	teleport_rot(portal);
}

void	teleport_pos_red(int portal)
{
	cam()->pos.x = (double)env()->red.x;
	cam()->pos.y = (double)env()->red.y;
	if (ft_abs(env()->sidered) == 1)
	{
		cam()->pos.x += (env()->sidered == -1) ? 1.0 : 0;
		cam()->pos.y += 0.5;
	}
	else
	{
		cam()->pos.y += (env()->sidered == -2) ? 1.0 : 0;
		cam()->pos.x += 0.5;
	}
	teleport_rot(portal);
}

void	teleport_pos(int portal)
{
	if (portal == 3 && env()->sideblue != 0)
		teleport_pos_blue(portal);
	else if (portal == 4 && env()->sidered != 0)
		teleport_pos_red(portal);
}
