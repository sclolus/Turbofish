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
#include <stdio.h>

t_key	*key(void)
{
	static t_key k;

	return (&k);
}

int		ft_key_pressed(int keycode, void *param)
{
	printf("key pressed %d ", keycode);
	(void)param;
	if (keycode == KEY_UP) {
		printf("up");
		key()->up = 1;
	}
	if (keycode == KEY_DOWN) {
		printf("down");
		key()->down = 1;
	}
	if (keycode == KEY_RIGHT) {
		printf("right");
		key()->right = 1;
	}
	if (keycode == KEY_LEFT) {
		printf("left");
		key()->left = 1;
	}
	if (keycode == KEY_ESCAPE)
	{
		/* system("killall afplay 2> /dev/null"); */
		exit(0);
	}
	printf("\n");
	return (0);
}

int		ft_key_release(int keycode, void *param)
{
	(void)param;
	printf("key release %d", keycode);
	if (keycode == KEY_UP) {
		printf("up");
		key()->up = 0;
	}
	if (keycode == KEY_DOWN) {
		printf("down");
		key()->down = 0;
	}
	if (keycode == KEY_RIGHT) {
		printf("right");
		key()->right = 0;
	}
	if (keycode == KEY_LEFT) {
		printf("left");
		key()->left = 0;
	}
	if (keycode == KEY_Z || keycode == KEY_S)
	{
		if (keycode == KEY_Z) {
			printf("Z");
		}
		else if (keycode == KEY_S) {
			printf("S");
		}
		portal_gun_shoot(keycode);
		/* system("afplay sound/portal_gun_shoot.mp3 &"); */
	}
	printf("\n");
	return (0);
}
