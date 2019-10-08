/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   s_logical_b.c                                      :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/04/10 18:33:41 by bmickael          #+#    #+#             */
/*   Updated: 2017/04/10 22:57:49 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "internal_printf.h"

static void		f_b(char *s, uintmax_t n, int size)
{
	while (size)
	{
		s[--size] = (n & 1) ? '1' : '0';
		n >>= 1;
	}
}

static void		buffer_b(uintmax_t n, t_args *args, int *params, t_status *op)
{
	int flag_0;
	int flag_hash;
	int left_justify;
	int start_n;

	left_justify = (args->b & MINUS) ? 1 : 0;
	flag_hash = (args->b & HASH) ? 2 : 0;
	flag_0 = ((args->b & ZERO) && (args->p == -1) && (!(left_justify))) ? 1 : 0;
	start_n = (left_justify) ? 0 : params[1] - params[0];
	if (!assign_segment(params[1], op))
		return ;
	ft_memset(op->ptr, (flag_0) ? '0' : ' ', params[1]);
	ft_memcpy(op->ptr + ((flag_0) ? 0 : start_n), "0b", flag_hash);
	start_n += flag_hash;
	f_b(op->ptr + start_n, n, params[0] - flag_hash);
}

void			s_logical_b(t_args *args, t_status *op)
{
	uintmax_t	n;
	int			params[2];

	n = va_arg(op->ap, uintmax_t);
	args->b &= (n == 0) ? 0b11111001 : 0xFF;
	cast_u(&n, args->l);
	params[0] = 1;
	while ((params[0] < 64) && n >> params[0])
		params[0] += 1;
	params[0] = (args->p == 0 && !n) ? 0 : params[0];
	params[0] = (args->p > params[0]) ? \
	args->p : params[0];
	params[0] += ((args->b & HASH) && n) ? 2 : 0;
	params[1] = ((int)args->w > params[0]) ? args->w : params[0];
	buffer_b(n, args, params, op);
}
