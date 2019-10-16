/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   move_sprites.c                                     :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/07/03 18:55:23 by bmickael          #+#    #+#             */
/*   Updated: 2017/07/03 18:55:25 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include <stdio.h>
#include <stdlib.h>
#include "core/wolf3d.h"

/*
** TODO Brouillon en cours...
** make_decision consulte les degres de liberte.
*/

static void		make_decision(t_sprite *sprite, t_tile **tile, long int t)
{
	t_coord_f	res[5];
	int			i;

	sprite->last_time = t;
	sprite->origin = sprite->location;
	i = -1;
	while (++i < 5)
		res[i] = sprite->location;
	i = 1;
	if (tile[(int)sprite->location.y + 1][(int)sprite->location.x].value < 50)
		res[i++].y += 1;
	if (tile[(int)sprite->location.y - 1][(int)sprite->location.x].value < 50)
		res[i++].y -= 1;
	if (tile[(int)sprite->location.y][(int)sprite->location.x + 1].value < 50)
		res[i++].x += 1;
	if (tile[(int)sprite->location.y][(int)sprite->location.x - 1].value < 50)
		res[i++].x -= 1;
	sprite->goal = res[rand() % i];
}

void			init_sprite_ai(t_env *e)
{
	int			i;
	long int	t;

	i = 0;
	t = get_time();
	while (i < e->n_sprites)
	{
		make_decision(&e->sprites[i], e->map_tiles, t);
		i++;
	}
}

void			animate_sprites(t_env *e)
{
	long int	nev;
	int			i;

	nev = get_time();
	i = 0;
	while (i < e->n_sprites)
	{
		if (nev - e->sprites[i].last_time > 500)
		{
			e->sprites[i].location.x = e->sprites[i].goal.x;
			e->sprites[i].location.y = e->sprites[i].goal.y;
			make_decision(&e->sprites[i], e->map_tiles, nev +
									(nev - e->sprites[i].last_time) - 500);
		}
		else
		{
			e->sprites[i].location.x = e->sprites[i].origin.x +
				(nev - e->sprites[i].last_time) * (e->sprites[i].goal.x -
				e->sprites[i].origin.x) / 500;
			e->sprites[i].location.y = e->sprites[i].origin.y +
				(nev - e->sprites[i].last_time) * (e->sprites[i].goal.y -
				e->sprites[i].origin.y) / 500;
		}
		i++;
	}
}
