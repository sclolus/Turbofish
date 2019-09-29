/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   env.c                                              :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: rbadia <rbadia@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2016/12/27 12:27:53 by rbadia            #+#    #+#             */
/*   Updated: 2017/05/02 13:40:33 by vcombey          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "wolf.h"
#include <unistd.h>
#include <mlx.h>

int		init_env(char *file)
{
	(void)file;
	env()->win = NULL;
	env()->img = NULL;
	env()->mlx = NULL;
	ft_parse_input(file);
	if (!env()->mlx && !(env()->mlx = mlx_init()))
		return (ft_retmsg("mlx problem", 2));
	if (!env()->win && !(env()->win = mlx_new_window(env()->mlx, SCREEN_WIDTH,
	SCREEN_HEIGHT, "Wolf3d")))
		return (ft_retmsg("mlx problem", 2));
	if (!env()->img && !(env()->img = mlx_new_image(env()->mlx, SCREEN_WIDTH,
	SCREEN_HEIGHT)))
		return (ft_retmsg("mlx problem", 2));
	env()->ptr = mlx_get_data_addr(env()->img, &env()->bpp, &env()->size_line,
	&env()->endian);
	env()->life = 100;
	env()->sideblue = 0;
	env()->sidered = 0;
	return (0);
}
