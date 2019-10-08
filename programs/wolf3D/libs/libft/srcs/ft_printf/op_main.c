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
	if (!assign_segment(size, op))
		return ;
	ft_memset(op->ptr, (args->b & ZERO && !(args->b & MINUS)) ?
		'0' : ' ', size);
	op->ptr[(args->b & MINUS) ? 0 : size - 1] = c;
}

static void		add_args(t_status *op, int *i)
{
	t_args args;

	if (*op->s == '{')
		assign_modifier(op);
	else
	{
		if (extract_args(op->s + 1, &args, i, op))
		{
			op->s += *i + 1;
			args.f(&args, op);
		}
		else
		{
			op->s += *i + 1;
			if (*op->s)
				osx_nospecifier(&args, op, *op->s++);
			else if (!assign_segment(0, op))
				return ;
		}
	}
}

int				assign_segment(int w_size, t_status *op)
{
	op->size += w_size;
	new_chain(op);
	if (!op->ptr)
		return (FALSE);
	op->ptr -= w_size;
	return (TRUE);
}

void			new_chain(t_status *op)
{
	int			i;
	const char	*tmp;

	i = 0;
	while (op->s[i] != '%' && op->s[i] != 0x00 && op->s[i] != '{')
		i++;
	if (i)
	{
		tmp = op->s;
		op->s = op->s + i;
		if (!assign_segment(i, op))
			return ;
		ft_memcpy(op->ptr, tmp, i);
		return ;
	}
	else if (op->s[i] != 0x00)
	{
		add_args(op, &i);
		return ;
	}
	if (!(op->ptr = (char *)malloc((op->size + 1))))
		return ;
	op->ptr += op->size;
	*(op->ptr) = '\0';
}
