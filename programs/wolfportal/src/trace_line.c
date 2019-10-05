/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   trace.c                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: vcombey <marvin@42.fr>                     +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2016/12/12 22:04:33 by vcombey           #+#    #+#             */
/*   Updated: 2017/04/28 12:29:36 by vcombey          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "wolf.h"

int		ft_round(float a)
{
	return ((int)(a + 0.5));
}

void	ft_trace_puppy_line(t_double_pos a, t_double_pos b)
{
	int		i;
	float	m;
	float	j;
	int		s;
	int		c;

	c = 0;
	i = a.x < b.x ? ft_round(a.x) : ft_round((int)b.x);
	s = ft_round(b.x + a.x - i);
	m = (b.y - a.y) / (b.x - a.x);
	j = m * i + a.y - m * a.x;
	while (i < s)
	{
		ft_pixelput(ft_round(j), i, 0xFF0000);
		i++;
		c++;
		j += m;
	}
}

void	ft_trace_vertical_line(t_double_pos a, t_double_pos b)
{
	int		i;
	int		s;

	i = a.x < b.x ? a.x : b.x;
	s = ft_round(b.x + a.x - i);
	while (i < s)
	{
		ft_pixelput(ft_round(a.y), i, 0xFF0000);
		i++;
	}
	return ;
}

void	ft_trace_line(t_double_pos a, t_double_pos b)
{
	int		i;
	float	m;
	int		s;
	float	j;

	if (a.y == b.y)
		ft_trace_vertical_line(a, b);
	i = a.y < b.y ? ft_round(a.y) : ft_round((int)b.y);
	s = ft_round(b.y + a.y - i);
	m = (b.x - a.x) / (b.y - a.y);
	if (m > 1 || m < -1)
	{
		ft_trace_puppy_line(a, b);
		return ;
	}
	j = m * i + a.x - m * a.y;
	while (i < s)
	{
		ft_pixelput(i, ft_round(j), 0xFF0000);
		i++;
		j += m;
	}
}
