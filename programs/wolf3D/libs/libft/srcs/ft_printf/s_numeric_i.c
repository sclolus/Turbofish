/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   s_numeric_i.c                                      :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/04/10 18:50:23 by bmickael          #+#    #+#             */
/*   Updated: 2017/04/10 18:52:10 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "internal_printf.h"

static void		f_i(char *s, intmax_t n, int size)
{
	if (n < 0)
	{
		while (size--)
		{
			s[size] = (-(n % 10)) + '0';
			n /= 10;
		}
	}
	else
	{
		while (size--)
		{
			s[size] = (n % 10) + '0';
			n /= 10;
		}
	}
}

static void		buffer_i(intmax_t n, int *size, t_args *args, t_status *op)
{
	int x;
	int flag_0;

	if (!assign_segment(size[1], op))
		return ;
	flag_0 = ((args->b & ZERO) && (!(args->b & MINUS)) && args->p == -1) ?
																		1 : 0;
	x = ((n < 0) || (args->b & PLUS) || (args->b & SPACE)) ? 1 : 0;
	if (args->b & MINUS)
		ft_memset(op->ptr + size[0] + x, ' ', size[1] - size[0] - x);
	else
		ft_memset(op->ptr + ((flag_0 && x) ? 1 : 0),
			flag_0 ? '0' : ' ', size[1] - size[0] - x);
	if (n < 0)
		op->ptr[(flag_0 || args->b & MINUS) ? 0 : size[1] - size[0] - 1] = '-';
	else if (args->b & PLUS)
		op->ptr[(flag_0 || args->b & MINUS) ? 0 : size[1] - size[0] - 1] = '+';
	else if (args->b & SPACE)
		op->ptr[(flag_0 || args->b & MINUS) ? 0 : size[1] - size[0] - 1] = ' ';
	f_i(op->ptr + ((args->b & MINUS) ? x : size[1] - size[0]), n, size[0]);
}

void			s_numeric_i(t_args *args, t_status *op)
{
	intmax_t	n;
	intmax_t	i;
	int			size[2];
	int			x;

	n = va_arg(op->ap, intmax_t);
	cast_i(&n, args->l);
	size[0] = (args->p == 0 && n == 0) ? 0 : 1;
	i = n;
	while ((i = i / 10))
		size[0]++;
	size[0] = (args->p > size[0]) ? args->p : size[0];
	x = ((n < 0) || (args->b & PLUS) || (args->b & SPACE)) ? 1 : 0;
	size[1] = (args->w > (size[0] + x)) ? args->w : size[0] + x;
	buffer_i(n, size, args, op);
}
