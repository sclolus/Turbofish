/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   ft_pixelget.c                                      :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: vcombey <marvin@42.fr>                     +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2016/12/30 20:00:04 by vcombey           #+#    #+#             */
/*   Updated: 2017/04/25 16:36:29 by vcombey          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include <mlx.h>
#include "wolf.h"

unsigned int		ft_pixelget(int x, int y, t_texture t)
{
	int				dest;

	if (y < 0 || x < 0)
		return (0);
	dest = y * t.size_line + x * (t.bpp / 8);
	if ((t.height * t.width * (t.bpp / 8)) <= dest)
		return (0);
	if (dest < 0)
		return (0);
	return (*(unsigned int *)(&t.ptr[dest]));
}

unsigned int		ft_pixelget_img(int x, int y)
{
	int				dest;

	if (y < 0 || x < 0)
		return (0);
	dest = y * env()->size_line + x * (env()->bpp / 8);
	if (dest < 0)
		return (0);
	return (*(unsigned int *)(&env()->ptr[dest]));
}
