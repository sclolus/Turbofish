/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   s_numeric_u.c                                      :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/04/10 18:53:06 by bmickael          #+#    #+#             */
/*   Updated: 2017/04/10 18:53:57 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "internal_printf.h"

static void		f_u(char *s, uintmax_t n, int size)
{
	while (size--)
	{
		s[size] = (n % 10) + '0';
		n /= 10;
	}
}

static void		buffer_u(uintmax_t n, int *size, t_args *args, t_status *op)
{
	int i;
	int j;

	if (!assign_segment(size[1], op))
		return ;
	if (args->b & MINUS)
	{
		i = size[0];
		j = size[1];
		while (i < j)
			op->ptr[i++] = ' ';
		f_u(op->ptr, n, size[0]);
		return ;
	}
	i = 0;
	j = size[1] - size[0];
	if ((args->b & ZERO) && (args->p == -1))
		while (i < j)
			op->ptr[i++] = '0';
	else
		while (i < j)
			op->ptr[i++] = ' ';
	f_u(op->ptr + j, n, size[0]);
}

void			s_numeric_u(t_args *args, t_status *op)
{
	int			size[2];
	uintmax_t	n;
	uintmax_t	i;

	n = va_arg(op->ap, uintmax_t);
	cast_u(&n, args->l);
	size[0] = (args->p == 0 && !n) ? 0 : 1;
	i = n;
	while ((i = i / 10))
		(size[0])++;
	size[0] = (args->p > size[0]) ? args->p : size[0];
	size[1] = (args->w > size[0]) ? args->w : size[0];
	buffer_u(n, size, args, op);
}
