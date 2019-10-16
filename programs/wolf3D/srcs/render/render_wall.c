/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   render_wall.c                                      :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: stoupin <stoupin@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/07/06 11:10:18 by stoupin           #+#    #+#             */
/*   Updated: 2018/02/01 17:02:01 by stoupin          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include <stdlib.h>
#include "render.h"

void	init_walls(t_env *e, char **textures, int n)
{
	if (!(e->scene.bmp_wall = load_bitmap(textures, n))) {
		exit(EXIT_FAILURE);
	}
}

void	render_wall(t_env *e, t_coord_i c, t_coord_f angle)
{
	t_column	*cl;
	t_coord_f	uv;
	int			type;
	float		dist;

	(void)angle;
	cl = &(e->scene.columns[c.x]);
	type = cl->type;
	uv.y = (e->player.height + cl->wall_h_dist * e->atan_list[c.y]) / e->wall_height;
	uv.x = -cl->wall_x_tex / 4.f * (e->scene.bmp_wall[type].dim.x - 1);
	uv.y = (1.f - uv.y) * (e->scene.bmp_wall[type].dim.y - 1);
	dist = cl->wall_h_dist;
	e->scene.scene[c.y * WIDTH + c.x] = get_pix(&(e->scene.bmp_wall[type]), uv, dist, e->inter_state);
}
