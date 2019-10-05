/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   ft_rot_pos.c                                       :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: vcombey <marvin@42.fr>                     +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/04/25 21:15:56 by vcombey           #+#    #+#             */
/*   Updated: 2017/04/26 14:36:40 by vcombey          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "wolf.h"

void	ft_swap_double_pos(t_double_pos *a)
{
	double	tmp;

	tmp = a->y;
	a->y = a->x;
	a->x = tmp;
}

void	ft_rev_rot_int(t_int_pos *a)
{
	int		tmp;

	tmp = a->y;
	a->y = -a->x;
	a->x = tmp;
}

void	ft_rev_rot_double(t_double_pos *a)
{
	double	tmp;

	tmp = a->y;
	a->y = -a->x;
	a->x = tmp;
}

void	ft_rot_int(t_int_pos *a)
{
	int		tmp;

	tmp = -a->y;
	a->y = a->x;
	a->x = tmp;
}

void	ft_rot_double(t_double_pos *a)
{
	double	tmp;

	tmp = -a->y;
	a->y = a->x;
	a->x = tmp;
}
