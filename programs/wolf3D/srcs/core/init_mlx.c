/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   mlx_init.c                                         :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/05/12 03:59:23 by bmickael          #+#    #+#             */
/*   Updated: 2017/06/03 16:03:48 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include <stdlib.h>
#include "core/wolf3d.h"

int				init_mlx(t_env *env, char *window_name, int width, int height)
{
	if (!(env->mlx = mlx_init()))
		return (1);
	if (!(env->win = mlx_new_window(env->mlx, width, height, window_name)))
		return (1);
	return (0);
}

int				exit_mlx(t_env *e)
{
	mlx_destroy_image(e->mlx, e->image);
	mlx_destroy_window(e->mlx, e->win);
	exit(EXIT_SUCCESS);
	return (0);
}
