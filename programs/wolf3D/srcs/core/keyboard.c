/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   keyboard.c                                         :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/05/13 03:51:41 by bmickael          #+#    #+#             */
/*   Updated: 2017/06/03 16:00:22 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include <unistd.h>
#include <stdlib.h>
#include "core/wolf3d.h"

#ifdef DEBUG_KEYBOARD

static void	print_register(unsigned long int *k)
{
	int i;

	i = 0;
	while (i < 256)
	{
		if (k[i])
			ft_printf("{green}%c{eoc}", (k[i]) ? '1' : '0');
		else
			ft_putchar(k[i] + '0');
		i++;
	}
	write(1, "\n", 1);
}

#endif

int			mlx_key_release(int keycode, t_env *e)
{
	e->keyb[keycode & 0xFF] = 0;
	if (DEBUG_KEYBOARD)
	{
		ft_printf("keycode %3i [mod]%3i RELEASED -> ", keycode, keycode & 0xFF);
		print_register(e->keyb);
	}
	return (keycode);
}

int			mlx_key_press(int keycode, t_env *e)
{
	if (!e->keyb[keycode & 0xFF])
		e->keyb[keycode & 0xFF] = get_time();
	if (DEBUG_KEYBOARD)
	{
		ft_printf("keycode %3i [mod]%3i PRESSED -> ", keycode, keycode & 0xFF);
		print_register(e->keyb);
	}
	return (keycode);
}

void		interpolate_switch(t_env *e, unsigned long int m)
{
	char *s;

	if ((m - e->inter_time) < 1000)
	{
		ft_asprintf(&s, "Interpolation lineaire: %s",
									(e->inter_state) ? "ON" : "OFF");
		mlx_string_put(e->mlx, e->win, 20, 40, 0x00FFFFFF, s);
		free(s);
	}
	else
		e->inter_time = 0;
}
