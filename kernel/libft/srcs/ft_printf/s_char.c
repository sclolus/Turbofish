/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   s_char.c                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/04/29 01:47:07 by bmickael          #+#    #+#             */
/*   Updated: 2017/04/29 01:47:09 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "internal_printf.h"

int				get_size_for_string(wchar_t c)
{
	if (c <= 0x7F || (c <= 0xFF && (MB_CUR_MAX == 1)))
		return (1);
	else if (c <= 0x7FF)
		return (2);
	else if (c <= 0xFFFF)
		return (3);
	return (4);
}

static int		check_valid(wchar_t c)
{
	if ((c & 0x80000000) || (c >= 0xD800 && c <= 0xDFFF))
		return (0);
	if (MB_CUR_MAX == 2 && (c <= (wchar_t)0x7FF))
		return (1);
	else if (MB_CUR_MAX == 3 && (c <= (wchar_t)0xFFFF))
		return (1);
	else if (MB_CUR_MAX == 4 && (c <= (wchar_t)0x10FFFF))
		return (1);
	return (0);
}

int				s_char(t_args *args, t_status *op)
{
	int		size;
	wchar_t	c;

	c = (wchar_t)va_arg(op->ap, wchar_t);
	if ((args->l & L) && !(MB_CUR_MAX == 1 && (c >= 0 && c <= 0xFF)))
	{
		if (!(check_valid(c)))
			return (-1);
		s_char_wchar(args, op, c);
		return (0);
	}
	size = (args->w) ? args->w : 1;
	if (args->b & MINUS)
	{
		char_to_buffer(c, 1, op);
		char_to_buffer(' ', size - 1, op);
		return (0);
	}
	char_to_buffer((args->b & ZERO) ? '0' : ' ', size - 1, op);
	char_to_buffer(c, 1, op);
	return (0);
}
