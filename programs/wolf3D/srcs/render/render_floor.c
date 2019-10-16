/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   render_floor.c                                     :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: stoupin <stoupin@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/07/06 11:10:08 by stoupin           #+#    #+#             */
/*   Updated: 2018/02/01 14:10:26 by stoupin          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include <stdlib.h>
#include "render.h"

void	init_floor(t_env *e, char **textures, int n)
{
	if (!(e->scene.bmp_floor = load_bitmap(textures, n)))
		exit(EXIT_FAILURE);
}

void	render_floor(t_env *e, t_coord_i c, t_coord_f angle)
{
	float		dist;
	t_coord_f	uv;
	int			type;

	dist = e->dist_floor[c.y] / e->cos_list[c.x];
	uv.x = (e->player.location.x + dist * cosf(angle.x)) / 4.f;
	uv.y = (e->player.location.y + dist * sinf(angle.x)) / 4.f;
	type = e->map_tiles[(int)uv.y][(int)uv.x].value;
	if (type >= 50)
		type = 0;
	uv.x = (uv.x - floorf(uv.x)) * (e->scene.bmp_floor[type].dim.x - 1);
	uv.y = (uv.y - floorf(uv.y)) * (e->scene.bmp_floor[type].dim.y - 1);
	e->scene.scene[c.y * WIDTH + c.x] = get_pix(&(e->scene.bmp_floor[type]),
													uv, dist, e->inter_state);
}
