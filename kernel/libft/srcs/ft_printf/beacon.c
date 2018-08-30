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

int		ft_printf(const char *format, ...)
{
	t_status	op;
	int			ret;

	ft_memset(&op, 0, sizeof(t_status));
	op.s = format;
	op.fd = STDOUT;
	va_start(op.ap, format);
	ret = new_chain(&op);
	va_end(op.ap);
	if (ret < 0)
		return (ret);
	fflush_buffer(&op);
	return (op.total_size);
}

int		ft_dprintf(bool display, const char *format, ...)
{
	t_status	op;
	int			ret;

	if (display == false)
		return (0);
	ft_memset(&op, 0, sizeof(t_status));
	op.s = format;
	op.fd = STDOUT;
	va_start(op.ap, format);
	ret = new_chain(&op);
	va_end(op.ap);
	if (ret < 0)
		return (ret);
	fflush_buffer(&op);
	return (op.total_size);
}

int		ft_eprintf(const char *format, ...)
{
	t_status	op;
	int			ret;

	ft_memset(&op, 0, sizeof(t_status));
	op.s = format;
	op.fd = STDERR;
	va_start(op.ap, format);
	ret = new_chain(&op);
	va_end(op.ap);
	if (ret < 0)
		return (ret);
	fflush_buffer(&op);
	return (op.total_size);
}

int		ft_fprintf(int const fd, const char *format, ...)
{
	t_status	op;
	int			ret;

	ft_memset(&op, 0, sizeof(t_status));
	op.s = format;
	op.fd = fd;
	va_start(op.ap, format);
	ret = new_chain(&op);
	va_end(op.ap);
	if (ret < 0)
		return (ret);
	fflush_buffer(&op);
	return (op.total_size);
}

int		ft_sprintf(char *str, const char *format, ...)
{
	t_status	op;
	int			ret;

	if (str == NULL)
		return (-1);
	ft_memset(&op, 0, sizeof(t_status));
	op.s = format;
	op.str = str;
	va_start(op.ap, format);
	ret = new_chain(&op);
	va_end(op.ap);
	if (ret < 0)
		return (ret);
	fflush_buffer(&op);
	return (op.total_size);
}
