/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   draw_portal_gun.c                                  :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: vcombey <marvin@42.fr>                     +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/04/26 14:49:16 by vcombey           #+#    #+#             */
/*   Updated: 2017/04/27 16:46:48 by vcombey          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "wolf.h"
#include <math.h>

void				draw_portal_gun(void)
{
	double			y;
	double			x;
	unsigned int	color;

	y = 3 * SCREEN_HEIGHT / 5;
	x = SCREEN_WIDTH / 2;
	while (y < SCREEN_HEIGHT)
	{
		x = SCREEN_WIDTH / 2;
		while (x < SCREEN_WIDTH)
		{
			if ((color = ft_pixelget((x / (SCREEN_WIDTH / 2) - 1) * gun()->width, ((y - (3 * SCREEN_HEIGHT / 5)) / (2 * SCREEN_HEIGHT / 5)) * gun()->height, *gun())) != 0x00FF00)
				ft_pixelput(x, y, color);
			x++;
		}
		y++;
	}
}
