/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   ft_pixelput.c                                      :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: vcombey <marvin@42.fr>                     +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2016/12/30 20:00:15 by vcombey           #+#    #+#             */
/*   Updated: 2016/12/30 20:00:17 by vcombey          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include <mlx.h>
#include "wolf.h"

void				ft_pixelput(int x, int y, int color)
{
	int				i;
	int				dest;
	unsigned int	new_color;

	if (y < 0 || x < 0)
		return ;
	dest = y * env()->size_line + x * (env()->bpp / 8);
	if ((SCREEN_HEIGHT * SCREEN_WIDTH * (env()->bpp / 8)) <= dest)
		return ;
	if (dest < 0)
		return ;
	/* i = -1; */
	*((unsigned int *)&env()->ptr[dest]) = color;
	/* 
	 * while (++i < env()->bpp / 8)
	 * {
	 * 	env()->ptr[dest + i] = color >> (i * 8) & 0xFF;
	 * }
	 */
}
