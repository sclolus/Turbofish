/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   portal_gun.c                                       :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: vcombey <vcombey@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/04/18 08:59:29 by vcombey           #+#    #+#             */
/*   Updated: 2017/05/02 12:06:38 by vcombey          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "wolf.h"
#include <math.h>

void	change_portal_red(void)
{
	if (env()->sidered != 0)
		env()->map[env()->red.x][env()->red.y] = 1;
	env()->map[env()->wall.x][env()->wall.y] = 3;
	env()->red.x = env()->wall.x;
	env()->red.y = env()->wall.y;
	if (env()->side == 0 && cam()->dir.x >= 0)
		env()->sidered = 1;
	else if (env()->side == 0 && cam()->dir.x < 0)
		env()->sidered = -1;
	else if (env()->side == 1 && cam()->dir.y >= 0)
		env()->sidered = 2;
	else if (env()->side == 1 && cam()->dir.y < 0)
		env()->sidered = -2;
}

void	change_portal_blue(void)
{
	/* printf("env->blue.x %d, %d\n", env()->wall.x, env()->wall.y); */
	if (env()->sideblue != 0)
		env()->map[env()->blue.x][env()->blue.y] = 1;
	env()->map[env()->wall.x][env()->wall.y] = 4;
	env()->blue.x = env()->wall.x;
	env()->blue.y = env()->wall.y;
	if (env()->side == 0 && cam()->dir.x >= 0)
		env()->sideblue = 1;
	else if (env()->side == 0 && cam()->dir.x < 0)
		env()->sideblue = -1;
	else if (env()->side == 1 && cam()->dir.y >= 0)
		env()->sideblue = 2;
	else if (env()->side == 1 && cam()->dir.y < 0)
		env()->sideblue = -2;
}

void	change_portail(int keycode)
{
	/* printf("keycode: %d, %d, %d\n", keycode, KEY_Z, KEY_S); */
	if (keycode == KEY_Z)
		change_portal_red();
	else if (keycode == KEY_S)
		change_portal_blue();
}

void	ft_shoot(t_dda dda, int keycode)
{
	int hit;

	hit = 0;
	env()->wall.x = (int)cam()->pos.x;
	env()->wall.y = (int)cam()->pos.y;
	while (hit == 0)
	{
		if ((dda.side_dist)->x < (dda.side_dist)->y)
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
		if ((env()->map[env()->wall.x][env()->wall.y]) > 0 &&
				env()->map[env()->wall.x][env()->wall.y] != 5)
			hit = 1;
	}
	change_portail(keycode);
}

void	portal_gun_shoot(int keycode)
{
	t_dda			dda;
	t_double_pos	ray_dir;
	t_double_pos	side_dist;
	t_double_pos	delta_dist;
	t_int_pos		step;

	dda.ray_dir = &ray_dir;
	dda.side_dist = &side_dist;
	dda.delta_dist = &delta_dist;
	dda.step = &step;
	(dda.delta_dist)->x = sqrt(1 + (cam()->dir.y * cam()->dir.y) /
			(cam()->dir.x * cam()->dir.x));
	(dda.delta_dist)->y = sqrt(1 + (cam()->dir.x * cam()->dir.x) /
			(cam()->dir.y * cam()->dir.y));
	(dda.ray_dir)->x = cam()->dir.x;
	(dda.ray_dir)->y = cam()->dir.y;
	ft_init_dist(dda);
	return (ft_shoot(dda, keycode));
}
