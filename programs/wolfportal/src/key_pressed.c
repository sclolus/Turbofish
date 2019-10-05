/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   key_pressed.c                                      :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: vcombey <marvin@42.fr>                     +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/04/26 21:37:03 by vcombey           #+#    #+#             */
/*   Updated: 2017/05/02 12:45:05 by vcombey          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "wolf.h"
#include <stdlib.h>

t_key	*key(void)
{
	static t_key k;

	return (&k);
}

int		ft_key_pressed(int keycode, void *param)
{
	(void)param;
	if (keycode == KEY_UP)
		key()->up = 1;
	if (keycode == KEY_DOWN)
		key()->down = 1;
	if (keycode == KEY_RIGHT)
		key()->right = 1;
	if (keycode == KEY_LEFT)
		key()->left = 1;
	if (keycode == KEY_ESCAPE)
	{
		/* system("killall afplay 2> /dev/null"); */
		exit(0);
	}
	return (0);
}

int		ft_key_release(int keycode, void *param)
{
	(void)param;
	if (keycode == KEY_UP)
	{
		key()->up = 0;
	}
	if (keycode == KEY_DOWN)
		key()->down = 0;
	if (keycode == KEY_RIGHT)
		key()->right = 0;
	if (keycode == KEY_LEFT)
		key()->left = 0;
	if (keycode == KEY_Z || keycode == KEY_S)
	{
		portal_gun_shoot(keycode);
		/* system("afplay sound/portal_gun_shoot.mp3 &"); */
	}
	return (0);
}
