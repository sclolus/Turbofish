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

int				s_char(t_args *args, t_status *op)
{
	int		size;
	char	c;

	c = (char)va_arg(op->ap, int);
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
