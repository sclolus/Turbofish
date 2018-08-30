/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   s_string.c                                         :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/04/10 19:14:51 by bmickael          #+#    #+#             */
/*   Updated: 2017/04/10 23:57:04 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "internal_printf.h"

int				s_string(t_args *args, t_status *op)
{
	int			size;
	char		*str;

	str = (char *)va_arg(op->ap, char *);
	str = (!str) ? "(null)" : str;
	size = ft_strlen(str);
	size = ((args->p < size) && (args->p != -1)) ? args->p : size;
	if (args->w <= size)
	{
		string_to_buffer(str, size, op);
		return (0);
	}
	if (args->b & MINUS)
	{
		string_to_buffer(str, size, op);
		char_to_buffer(' ', args->w - size, op);
		return (0);
	}
	char_to_buffer(args->b & ZERO ? '0' : ' ', args->w - size, op);
	string_to_buffer(str, size, op);
	return (0);
}
