/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   find_wall.c                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: stoupin <stoupin@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/07/06 11:43:04 by stoupin           #+#    #+#             */
/*   Updated: 2018/02/02 10:16:51 by stoupin          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include <math.h>
#include "core/wolf3d.h"
#include "render.h"

static void	wall_finder_init(t_wall_finder *wf)
{
	wf->ray_dir = (t_coord_f){cosf(wf->angle), sinf(wf->angle)};
	wf->c_i = (t_coord_i){(int)(wf->player.x), (int)(wf->player.y)};
	wf->d_dist = (t_coord_f){0.f, 0.f};
	if (wf->ray_dir.x != 0.f)
		wf->d_dist.x = fabs(1.f / wf->ray_dir.x);
	if (wf->ray_dir.y != 0.f)
		wf->d_dist.y = fabs(1.f / wf->ray_dir.y);
}

static void	wall_finder_init_step(t_wall_finder *wf)
{
	wf->step = (t_coord_i){0, 0};
	wf->side_dist = (t_coord_f){0.f, 0.f};
	if (wf->ray_dir.x < 0.f)
	{
		wf->step.x = -1;
		wf->side_dist.x = (wf->player.x - (float)(wf->c_i.x)) * wf->d_dist.x;
	}
	else if (wf->ray_dir.x > 0.f)
	{
		wf->step.x = 1;
		wf->side_dist.x = ((float)(wf->c_i.x) + 1.f - wf->player.x)
							* wf->d_dist.x;
	}
	if (wf->ray_dir.y < 0.f)
	{
		wf->step.y = -1;
		wf->side_dist.y = (wf->player.y - (float)(wf->c_i.y))
												* wf->d_dist.y;
	}
	else if (wf->ray_dir.y > 0.f)
	{
		wf->step.y = 1;
		wf->side_dist.y = ((float)(wf->c_i.y) + 1.f - wf->player.y)
												* wf->d_dist.y;
	}
}

static int	wall_finder_exec(t_wall_finder *wf)
{
	int	hit;
	int	side;

	hit = 0;
	while (hit == 0)
	{
		if (wf->ray_dir.y == 0.f || wf->side_dist.x < wf->side_dist.y)
		{
			wf->side_dist.x += wf->d_dist.x;
			wf->c_i.x += wf->step.x;
			side = 0;
		}
		else
		{
			wf->side_dist.y += wf->d_dist.y;
			wf->c_i.y += wf->step.y;
			side = 1;
		}
		if (wf->tiles[wf->c_i.y][wf->c_i.x].value >= 50)
			hit = 1;
	}
	return (side);
}

/*
** This function has been modified for the new wolf3d subject (circa late 2017)
** x_tex is now in the range [0, 4[ instead of [0, 1[
** Meanwhile, wall textures are now 4 times larger, the new range binding to
** the four sides of the wall blocks.
*/

static void	wall_finder_intersect(t_wall_finder *wf, int side,
									t_coord_f *intersect, float *x_tex)
{
	double wall_dist;

	if (side == 0)
		wall_dist = ((float)(wf->c_i.x) - wf->player.x +
							(1.f - (float)(wf->step.x)) / 2.f) / wf->ray_dir.x;
	else
		wall_dist = ((float)(wf->c_i.y) - wf->player.y +
							(1.f - (float)(wf->step.y)) / 2.f) / wf->ray_dir.y;
	intersect->x = wf->player.x + wall_dist * wf->ray_dir.x;
	intersect->y = wf->player.y + wall_dist * wf->ray_dir.y;
	*x_tex = -1.f;
	if (side == 1)
	{
		*x_tex += (intersect->x - (float)wf->c_i.x) * wf->step.y;
		if ((float)wf->c_i.y >= intersect->y - 0.5)
			*x_tex -= 3.;
	}
	else
	{
		*x_tex += (1.f - intersect->y + (float)wf->c_i.y) * wf->step.x;
		if ((float)wf->c_i.x < intersect->x - 0.5)
			*x_tex -= 1.;
	}
}

/*
** This function finds the intersection between a ray and the first wall
** it encounters, along with the x uv coordinate on the texture
**	inputs:
**		env->map_tiles: 2d array representing the map (value >= 50 == wall)
**		env->player.location.x: x location of the player
**		env->player.location.y: y location of the player
**		angle_x: angle of the ray
**	outputs:
**		intersect: x, y coordinates of the intersection of the ray
**					with the wall
**		x_tex: x uv coordinate of the ray on the texture
*/

int			find_wall(t_env *env, float angle_x, t_coord_f *intersect,
						float *x_tex)
{
	t_wall_finder	wf;
	int				side;

	wf.tiles = env->map_tiles;
	wf.angle = angle_x;
	wf.player = env->player.location;
	wall_finder_init(&wf);
	wall_finder_init_step(&wf);
	side = wall_finder_exec(&wf);
	wall_finder_intersect(&wf, side, intersect, x_tex);
	return (env->map_tiles[wf.c_i.y][wf.c_i.x].value - 50);
}
