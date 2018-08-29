/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   op_main.c                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/04/10 18:27:01 by bmickael          #+#    #+#             */
/*   Updated: 2017/04/10 22:56:23 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "internal_printf.h"

static int		extract_args(const char *restrict s, t_args *args, int *i,
															t_status *op)
{
	ft_memset(args, 0, sizeof(t_args));
	args->p = -1;
	get_args(s, i, args, op);
	if (args->f == NULL)
		return (0);
	return (1);
}

static void		osx_nospecifier(t_args *args, t_status *op, char c)
{
	int size;

	size = (args->w) ? args->w : 1;
	if (args->b & MINUS)
	{
		char_to_buffer(c, 1, op);
		char_to_buffer(' ', size - 1, op);
		return ;
	}
	char_to_buffer(args->b & ZERO ? '0' : ' ', size - 1, op);
	char_to_buffer(c, 1, op);
}

static int		add_args(t_status *op, int *i)
{
	t_args args;

	if (*op->s == '{')
		assign_modifier(op);
	else
	{
		if (extract_args(op->s + 1, &args, i, op))
		{
			op->s += *i + 1;
			return (args.f(&args, op));
		}
		else
		{
			op->s += *i + 1;
			if (*op->s)
				osx_nospecifier(&args, op, *op->s++);
		}
	}
	return (0);
}

int				new_chain(t_status *op)
{
	int			i;
	int			ret;

	i = 0;
	while (op->s[i] != '\0')
	{
		while (op->s[i] != '%' && op->s[i] != '\0' && op->s[i] != '{')
			i++;
		if (i)
		{
			string_to_buffer(op->s, i, op);
			op->s = op->s + i;
		}
		else if (op->s[i] != '\0')
		{
			ret = add_args(op, &i);
			if (ret < 0)
				return (-1);
		}
		i = 0;
	}
	return (0);
}
