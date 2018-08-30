/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   norme.c                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/04/29 01:47:07 by bmickael          #+#    #+#             */
/*   Updated: 2017/04/29 01:47:09 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "internal_printf.h"

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

void			s_char_wchar(t_args *args, t_status *op, wchar_t c)
{
	int		size;
	char	tmp[4];
	int		utf8_s;

	utf8_s = get_size(c);
	size = (args->w > utf8_s) ? args->w : utf8_s;
	fill_wchar((wchar_t)c, tmp, utf8_s);
	if (args->b & MINUS)
	{
		string_to_buffer(tmp + (4 - utf8_s), utf8_s, op);
		char_to_buffer(' ', size - utf8_s, op);
		return ;
	}
	char_to_buffer(args->b & ZERO ? '0' : ' ', size - utf8_s, op);
	string_to_buffer(tmp + (4 - utf8_s), utf8_s, op);
}
