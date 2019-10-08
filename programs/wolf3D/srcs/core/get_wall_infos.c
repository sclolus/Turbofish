/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   get_wall_infos.c                                   :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/06/12 03:33:56 by bmickael          #+#    #+#             */
/*   Updated: 2017/06/12 03:34:06 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "core/wolf3d.h"

t_vector_2		create_vector(float angle, float dist)
{
	t_vector_2	s;

	s.module = dist;
	s.dx = cosf(angle) * dist;
	s.dy = sinf(angle) * dist;
	return (s);
}

t_wall_vector	end_seq(t_tile **tiles, float angle, t_wall_info i)
{
	int		side;

	side = 0;
	while (tiles[i.map.y][i.map.x].value < 50)
		if (i.side_dist.x < i.side_dist.y)
		{
			i.side_dist.x += i.delta_dist.x;
			i.map.x += i.step.x;
			side = 0;
		}
		else
		{
			i.side_dist.y += i.delta_dist.y;
			i.map.y += i.step.y;
			side = 1;
		}
	i.w.norm = (side == 1) ? (t_coord_f){0, -i.step.y} :
													(t_coord_f){-i.step.x, 0};
	if (side == 1)
		i.w.v = create_vector(angle,
		((float)i.map.y - i.ray_pos.y + (1. - i.step.y) / 2.) / i.ray_dir.y);
	else
		i.w.v = create_vector(angle,
		((float)i.map.x - i.ray_pos.x + (1. - i.step.x) / 2.) / i.ray_dir.x);
	return (i.w);
}

t_wall_vector	middle_seq(t_tile **tiles, float angle, t_wall_info i)
{
	i.delta_dist.x = sqrt(1. + (i.ray_dir.y * i.ray_dir.y) /
												(i.ray_dir.x * i.ray_dir.x));
	i.delta_dist.y = sqrt(1. + (i.ray_dir.x * i.ray_dir.x) /
												(i.ray_dir.y * i.ray_dir.y));
	if (i.ray_dir.x < 0)
	{
		i.step.x = -1;
		i.side_dist.x = (i.ray_pos.x - i.map.x) * i.delta_dist.x;
	}
	else
	{
		i.step.x = 1;
		i.side_dist.x = ((float)i.map.x + 1. - i.ray_pos.x) * i.delta_dist.x;
	}
	if (i.ray_dir.y < 0)
	{
		i.step.y = -1;
		i.side_dist.y = (i.ray_pos.y - i.map.y) * i.delta_dist.y;
	}
	else
	{
		i.step.y = 1;
		i.side_dist.y = ((float)i.map.y + 1. - i.ray_pos.y) * i.delta_dist.y;
	}
	return (end_seq(tiles, angle, i));
}

t_wall_vector	get_wall_info(t_tile **tiles, float angle, t_coord_f location)
{
	t_wall_info		i;

	i.ray_pos = (t_coord_f){location.x, location.y};
	i.ray_dir = (t_coord_f){cosf(angle), sinf(angle)};
	i.map = (t_coord_i){(int)i.ray_pos.x, (int)i.ray_pos.y};
	if (i.ray_dir.y == 0.)
	{
		i.step.x = (i.ray_dir.x > 0) ? 1 : -1;
		while (tiles[i.map.y][i.map.x].value < 50)
			i.map.x += i.step.x;
		i.w.norm = (t_coord_f){(i.ray_dir.x > 0) ? -1 : 1, 0};
		i.w.v = create_vector(angle, fabs((float)i.map.x - i.ray_pos.x));
		return (i.w);
	}
	if (i.ray_dir.x == 0.)
	{
		i.step.y = (i.ray_dir.y > 0) ? 1 : -1;
		while (tiles[i.map.y][i.map.x].value < 50)
			i.map.y += i.step.y;
		i.w.norm = (t_coord_f){0, (i.ray_dir.y > 0) ? -1 : 1};
		i.w.v = create_vector(angle, fabs((float)i.map.y - i.ray_pos.y));
		return (i.w);
	}
	return (middle_seq(tiles, angle, i));
}
