/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   define_mouvements.c                                :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/06/12 03:36:44 by bmickael          #+#    #+#             */
/*   Updated: 2017/06/12 03:36:55 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "core/wolf3d.h"

float			mvt_right(t_tile **map, t_coord_f mvt, t_coord_f location)
{
	t_wall_vector	w;

	w = get_wall_info(map, 0, (t_coord_f){location.x,
										location.y + mvt.y});
	if ((w.v.dx - 0.4) < mvt.x)
		return (w.v.dx - 0.4);
	else
		return (mvt.x);
}

float			mvt_left(t_tile **map, t_coord_f mvt, t_coord_f location)
{
	t_wall_vector	w;

	w = get_wall_info(map, PI, (t_coord_f){location.x,
										location.y + mvt.y});
	if (mvt.x < (w.v.dx + 0.4))
		return (w.v.dx + 0.4);
	else
		return (mvt.x);
}

float			mvt_top(t_tile **map, t_coord_f mvt, t_coord_f location)
{
	t_wall_vector	w;

	w = get_wall_info(map, PI * 3 / 2,
								(t_coord_f){location.x + mvt.x, location.y});
	if ((w.v.dy + 0.4) > mvt.y)
		return (w.v.dy + 0.4);
	else
		return (mvt.y);
}

float			mvt_back(t_tile **map, t_coord_f mvt, t_coord_f location)
{
	t_wall_vector	w;

	w = get_wall_info(map, PI * 1 / 2,
								(t_coord_f){location.x + mvt.x, location.y});
	if (mvt.y > (w.v.dy - 0.4))
		return (w.v.dy - 0.4);
	else
		return (mvt.y);
}
