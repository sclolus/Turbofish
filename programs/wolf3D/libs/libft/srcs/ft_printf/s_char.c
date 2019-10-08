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

static int		get_size(wchar_t c)
{
	if (c <= 0x7F)
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

static void		fill_wchar(wchar_t c, char tmp[4], int size)
{
	int i;
	int j;

	if (size == 1)
	{
		tmp[3] = c;
		return ;
	}
	i = 0;
	j = 1;
	while (j < size)
	{
		tmp[3 - i++] = 0x80 | (c & 0b111111);
		c = c >> 6;
		j++;
	}
	if (size == 2)
		tmp[3 - i] = 0xC0 | (c & 0b11111);
	else if (size == 3)
		tmp[3 - i] = 0xE0 | (c & 0b1111);
	else if (size == 4)
		tmp[3 - i] = 0xF0 | (c & 0b111);
}

void			s_char(t_args *args, t_status *op)
{
	char	tmp[4];
	int		size;
	int		utf8_s;
	wchar_t	c;

	c = (wchar_t)va_arg(op->ap, wchar_t);
	if (args->l & L && !(MB_CUR_MAX == 1 && (c >= 0 && c <= 0xFF)))
	{
		if (!(check_valid(c)))
			return ;
		utf8_s = get_size(c);
		if (!assign_segment((size = (args->w > utf8_s) ? args->w : utf8_s), op))
			return ;
		fill_wchar((wchar_t)c, tmp, utf8_s);
		ft_memset(op->ptr, (args->b & ZERO && !(args->b & MINUS)) ?
			'0' : ' ', size);
		ft_memcpy(op->ptr + ((args->b & MINUS) ? 0 : size - utf8_s),
			tmp + (4 - utf8_s), utf8_s);
		return ;
	}
	if (!assign_segment((size = (args->w) ? args->w : 1), op))
		return ;
	ft_memset(op->ptr, (args->b & ZERO && !(args->b & MINUS)) ?
		'0' : ' ', size);
	op->ptr[(args->b & MINUS) ? 0 : size - 1] = c;
}
