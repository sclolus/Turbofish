/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   floor_casting.c                                    :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: vcombey <marvin@42.fr>                     +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/01/04 14:03:14 by vcombey           #+#    #+#             */
/*   Updated: 2017/04/28 19:44:24 by vcombey          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "wolf.h"
#include <mlx.h>

t_texture		*floor_t(void)
{
	static t_texture	f;

	return (&f);
}

int				init_floor_texture(void)
{
	if (!(floor_t()->img = mlx_xpm_file_to_image(env()->mlx,
					"img/floor.bmp",
	&floor_t()->width, &floor_t()->height)))
		return (ft_retmsg("cannot load image floor.xpm :/", 2));
	floor_t()->ptr = mlx_get_data_addr(floor_t()->img, &floor_t()->bpp,
	&floor_t()->size_line, &floor_t()->endian);
	return (0);
}

static double	calc_wall_x_on_map(double dist_wall)
{
	double	wall_map_x;

	if (env()->side == 0)
		wall_map_x = cam()->pos.y + dist_wall * env()->ray_dir.y;
	else
		wall_map_x = cam()->pos.x + dist_wall * env()->ray_dir.x;
	wall_map_x -= (int)wall_map_x;
	return (wall_map_x);
}

static void		get_wall_bottom(double dist_wall, t_double_pos *wall_bottom)
{
	double	wall_map_x;

	wall_map_x = calc_wall_x_on_map(dist_wall);
	if (env()->side == 0 && env()->ray_dir.x > 0)
	{
		wall_bottom->x = env()->wall.x;
		wall_bottom->y = env()->wall.y + wall_map_x;
	}
	else if (env()->side == 0 && env()->ray_dir.x < 0)
	{
		wall_bottom->x = env()->wall.x + 1.0;
		wall_bottom->y = env()->wall.y + wall_map_x;
	}
	else if (env()->side == 1 && env()->ray_dir.y > 0)
	{
		wall_bottom->x = env()->wall.x + wall_map_x;
		wall_bottom->y = env()->wall.y;
	}
	else
	{
		wall_bottom->x = env()->wall.x + wall_map_x;
		wall_bottom->y = env()->wall.y + 1.0;
	}
}

void			floor_casting(int x, double dist_wall, int y)
{
	t_double_pos	wall_bottom;
	t_double_pos	curr_floor;
	double			dist_player;
	double			current_dist;
	double			weight;

	get_wall_bottom(dist_wall, &wall_bottom);
	dist_player = 0.0;
	if (y < 0)
		y = SCREEN_HEIGHT;
	while (++y < SCREEN_HEIGHT)
	{
		current_dist = SCREEN_HEIGHT / (2.0 * y - SCREEN_HEIGHT);
		weight = (current_dist - dist_player) / (dist_wall - dist_player);
		curr_floor.x = weight * wall_bottom.x + (1.0 - weight) * cam()->pos.x;
		curr_floor.y = weight * wall_bottom.y + (1.0 - weight) * cam()->pos.y;
		ft_pixelput(x, y, ft_pixelget((int)(curr_floor.x * floor_t()->height)
					% floor_t()->height,
		(int)(curr_floor.y * floor_t()->height) % floor_t()->height,
		*floor_t()));
		ft_pixelput(x, SCREEN_HEIGHT - y, ft_pixelget((int)(curr_floor.x *
						floor_t()->height)
		% floor_t()->height, (int)(curr_floor.y * floor_t()->height) %
		floor_t()->height, *floor_t()));
	}
}
