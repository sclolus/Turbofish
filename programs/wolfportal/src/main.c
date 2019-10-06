/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.c                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: vcombey <marvin@42.fr>                     +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2016/12/30 20:00:38 by vcombey           #+#    #+#             */
/*   Updated: 2017/05/02 12:45:47 by vcombey          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include <stddef.h>
#include <stdlib.h>
#include <errno.h>
#include <unistd.h>
#include <fcntl.h>
#include "libft.h"
#include <mlx.h>
#include "wolf.h"
#include <math.h>

int		quit(void *param)
{
	(void)param;
	/* system("killall afplay 2> /dev/null"); */
	exit(0);
	return (0);
}

/* 
 * int		main(int ac, char **av)
 * {
 * 	if (ac != 2)
 * 		return (ft_retmsg("Usage: ./wolf3d [map]", 2));
 * 	init_cam();
 * 	if (init_env(av[1]) == 1)
 * 		return (1);
 * 	init_wall_texture();
 * 	init_portal_gun_texture();
 * 	init_portal_blue_texture();
 * 	init_portal_red_texture();
 * 	init_tourelle_texture();
 * 	mlx_hook(env()->win, KEYPRESS, KEYPRESSMASK, &ft_key_pressed, NULL);
 * 	mlx_hook(env()->win, KEYRELEA, KEYRELEAMASK, &ft_key_release, NULL);
 * 	mlx_hook(env()->win, 17, 1, &quit, NULL);
 * 	ft_wolf();
 * 	mlx_put_image_to_window(env()->mlx, env()->win, env()->img, 0, 0);
 * 	mlx_put_image_to_window(env()->mlx, env()->win, texture()->img, 0, 0);
 * 	mlx_loop_hook(env()->mlx, recalc_img, NULL);
 * 	mlx_loop(env()->mlx);
 * 	return (0);
 * }
 */

int		main(int ac, char **av)
{
	/* 
	 * if (ac != 2)
	 * 	return (ft_retmsg("Usage: ./wolf3d [map]", 2));
	 */
	/* 
	 * init_cam();
	 * if (init_env(av[1]) == 1)
	 * 	return (1);
	 */
	/* 
	 * init_wall_texture();
	 * init_portal_gun_texture();
	 * init_portal_blue_texture();
	 * init_portal_red_texture();
	 * init_tourelle_texture();
	 */
	//TODO: remove this mlx_init
	mlx_init();
	mlx_hook(env()->win, KEYPRESS, KEYPRESSMASK, &ft_key_pressed, NULL);
	mlx_hook(env()->win, KEYRELEA, KEYRELEAMASK, &ft_key_release, NULL);
	mlx_hook(env()->win, 17, 1, &quit, NULL);
	/* 
	 * ft_wolf();
	 * mlx_put_image_to_window(env()->mlx, env()->win, env()->img, 0, 0);
	 * mlx_put_image_to_window(env()->mlx, env()->win, texture()->img, 0, 0);
	 * mlx_loop_hook(env()->mlx, recalc_img, NULL);
	 */
	mlx_loop(env()->mlx);
	return (0);
}
