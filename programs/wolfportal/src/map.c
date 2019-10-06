/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   map.c                                              :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: vcombey <vcombey@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2016/12/30 20:00:50 by vcombey           #+#    #+#             */
/*   Updated: 2017/04/28 16:57:08 by vcombey          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "wolf.h"
#include "libft.h"

void	display_map(void)
{
	int	y;
	int	x;

	y = 0;
	while (y < env()->map_height)
	{
		x = 0;
		while (x < env()->map_width)
		{
			ft_putnbr(env()->map[y][x]);
			x++;
		}
		ft_putchar('\n');
		y++;
	}
}
