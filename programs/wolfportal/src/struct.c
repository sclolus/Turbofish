/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   struct.c                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: vcombey <marvin@42.fr>                     +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/04/26 14:58:37 by vcombey           #+#    #+#             */
/*   Updated: 2017/04/28 18:57:53 by vcombey          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "wolf.h"
#include <unistd.h>
#include <mlx.h>

t_env		*env(void)
{
	static t_env	e;

	return (&e);
}

t_texture	*texture(void)
{
	static t_texture	t;

	return (&t);
}

t_texture	*gun(void)
{
	static t_texture	g;

	return (&g);
}

t_texture	*portal_blue(void)
{
	static t_texture	p;

	return (&p);
}

t_texture	*portal_red(void)
{
	static t_texture	p;

	return (&p);
}
