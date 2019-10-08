/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   s_logical_o.c                                      :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/04/10 18:37:27 by bmickael          #+#    #+#             */
/*   Updated: 2017/04/10 18:38:08 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "internal_printf.h"

static void		f_o(char *s, uintmax_t n, int size)
{
	while (size)
	{
		s[--size] = (n & 0b111) + '0';
		n >>= 3;
	}
}

static void		buffer_o(uintmax_t n, t_args *args, int *params, t_status *op)
{
	int flag_0;
	int left_justify;
	int start_n;

	left_justify = (args->b & MINUS) ? 1 : 0;
	flag_0 = ((args->b & ZERO) && (args->p == -1) && (!(left_justify))) ? 1 : 0;
	start_n = (left_justify) ? 0 : params[1] - params[0];
	if (!assign_segment(params[1], op))
		return ;
	if (left_justify)
		ft_memset(op->ptr + params[0], ' ', params[1] - params[0]);
	else
		ft_memset(op->ptr, (flag_0) ? '0' : ' ', params[1] - params[0] +
			((args->b & HASH) ? 1 : 0));
	if (args->b & HASH)
	{
		op->ptr[(flag_0) ? 0 : start_n] = '0';
		f_o(op->ptr + start_n + 1, n, params[0] - 1);
	}
	else
		f_o(op->ptr + start_n, n, params[0]);
}

void			s_logical_o(t_args *args, t_status *op)
{
	uintmax_t	n;
	uintmax_t	i;
	int			params[2];

	n = va_arg(op->ap, uintmax_t);
	cast_u(&n, args->l);
	params[0] = (!n && args->p == 0) ? 0 : 1;
	i = n;
	while ((i = i >> 3))
		params[0]++;
	if (n)
	{
		if (args->p <= params[0])
			params[0] += (args->b & HASH) ? 1 : 0;
		else
			args->b &= 0xFD;
	}
	else
		params[0] = (args->b & HASH) ? 1 : params[0];
	params[0] = (args->p > params[0]) ? args->p : params[0];
	params[1] = ((int)args->w > params[0]) ? args->w : params[0];
	buffer_o(n, args, params, op);
}
