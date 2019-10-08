/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   beacon.c                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/04/10 18:00:39 by bmickael          #+#    #+#             */
/*   Updated: 2017/04/10 18:02:28 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "internal_printf.h"

/*
**	'man 3 stdarg' to understand variadics macro.
*/

int		ft_printf(const char *restrict format, ...)
{
	t_status op;

	ft_memset(&op, 0, sizeof(t_status));
	op.s = format;
	va_start(op.ap, format);
	new_chain(&op);
	va_end(op.ap);
	if (op.ptr)
	{
		op.size = write(STDOUT, op.ptr, op.size);
		free(op.ptr);
		return (op.size);
	}
	return (-1);
}

int		ft_eprintf(const char *restrict format, ...)
{
	t_status op;

	ft_memset(&op, 0, sizeof(t_status));
	op.s = format;
	va_start(op.ap, format);
	new_chain(&op);
	va_end(op.ap);
	if (op.ptr)
	{
		op.size = write(STDERR, op.ptr, op.size);
		free(op.ptr);
		return (op.size);
	}
	return (-1);
}

int		ft_fprintf(int const fd, const char *restrict format, ...)
{
	t_status op;

	ft_memset(&op, 0, sizeof(t_status));
	op.s = format;
	va_start(op.ap, format);
	new_chain(&op);
	va_end(op.ap);
	if (op.ptr)
	{
		op.size = write(fd, op.ptr, op.size);
		free(op.ptr);
		return (op.size);
	}
	return (-1);
}

int		ft_asprintf(char **str, const char *restrict format, ...)
{
	t_status op;

	ft_memset(&op, 0, sizeof(t_status));
	op.s = format;
	va_start(op.ap, format);
	new_chain(&op);
	va_end(op.ap);
	if (!(*str = op.ptr))
		return (-1);
	return (op.size);
}

int		ft_sprintf(char *str, const char *restrict format, ...)
{
	t_status op;

	ft_memset(&op, 0, sizeof(t_status));
	op.s = format;
	va_start(op.ap, format);
	new_chain(&op);
	va_end(op.ap);
	if (op.ptr)
	{
		ft_memcpy(str, op.ptr, op.size);
		free(op.ptr);
		return (op.size);
	}
	return (-1);
}
