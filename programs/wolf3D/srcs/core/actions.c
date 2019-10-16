/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   set_coord.c                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/05/13 06:04:12 by bmickael          #+#    #+#             */
/*   Updated: 2017/05/18 04:40:21 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include <math.h>
#include <stdlib.h>
#include <stdbool.h>

#include "core/wolf3d.h"
#include "bmp/bmp.h"

t_coord_f		test_mvt(t_wall_vector w, t_env *e, t_coord_f new)
{
	if (w.norm.y == 1 && new.y < (w.v.dy + 0.4))
	{
		new.y = w.v.dy + 0.4;
		new.x = (new.x > 0) ? mvt_right(e->map_tiles, new, e->player.location) :
			mvt_left(e->map_tiles, new, e->player.location);
	}
	if (w.norm.x == 1 && new.x < (w.v.dx + 0.4))
	{
		new.x = w.v.dx + 0.4;
		new.y = (w.v.dy > 0) ? mvt_back(e->map_tiles, new, e->player.location) :
			mvt_top(e->map_tiles, new, e->player.location);
	}
	if (w.norm.y == -1 && new.y > (w.v.dy - 0.4))
	{
		new.y = w.v.dy - 0.4;
		new.x = (new.x > 0) ? mvt_right(e->map_tiles, new, e->player.location) :
			mvt_left(e->map_tiles, new, e->player.location);
	}
	if (w.norm.x == -1 && new.x > (w.v.dx - 0.4))
	{
		new.x = w.v.dx - 0.4;
		new.y = (w.v.dy > 0) ? mvt_back(e->map_tiles, new, e->player.location) :
			mvt_top(e->map_tiles, new, e->player.location);
	}
	return (new);
}

static void		set_player_data(t_env *e, float q, float l)
{
	t_coord_f			new;
	t_wall_vector		w;

	e->player.angle += q * PI / 360;
	if (e->player.angle < 0)
		e->player.angle += 2.f * PI;
	else if (e->player.angle >= 2.f * PI)
		e->player.angle -= 2.f * PI;
	new.x = (cosf(e->player.angle)) * l;
	new.y = (sinf(e->player.angle)) * l;
	w = get_wall_info(e->map_tiles, e->player.angle, e->player.location);
	new = test_mvt(w, e, new);
	w = get_wall_info(e->map_tiles, e->player.angle + PI, e->player.location);
	new = test_mvt(w, e, new);
	e->player.location.x += new.x;
	e->player.location.y += new.y;
}

int				move_player(t_env *e)
{
	int							i;
	static t_modify_coord		types[N_CONTROL] = {
		{KEYB_ARROW_LEFT, KEYB_MMO_A, -0.20, 0},
		{KEYB_ARROW_RIGHT, KEYB_MMO_D, +0.20, 0},
		{KEYB_ARROW_UP, KEYB_MMO_W, 0, 0.015},
		{KEYB_ARROW_DOWN, KEYB_MMO_S, 0, -0.015}
	};
	unsigned long int			time_elapsed;
	float						new_q;
	float						new_l;

	i = -1;
	while (++i < N_CONTROL)
		if (e->keyb[types[i].keycode_1])
		{
			time_elapsed = get_time();
			new_q = (time_elapsed - e->keyb[types[i].keycode_1]) * types[i].q;
			new_l = (time_elapsed - e->keyb[types[i].keycode_1]) * types[i].l;
			e->keyb[types[i].keycode_1] = time_elapsed;
			set_player_data(e, new_q, new_l);
		}
	return (0);
}

static int		event_register(t_env *e, int keycode, int *state)
{
	static int	reg[512];

	keycode &= 0x1FF;
	if (e->keyb[keycode] && reg[keycode] == false)
	{
		reg[keycode] = true;
		*state = true;
		return (1);
	}
	else if (!(e->keyb[keycode]) && reg[keycode] == true)
		reg[keycode] = false;
	return (0);
}

int				common_action(t_env *e)
{
	int state;

	state = false;
	if (e->keyb[KEYB_ESCAPE])
		exit_mlx(e);
	if (event_register(e, KEYB_M, &state))
		e->display_minimap = (e->display_minimap) ? false : true;
	if (event_register(e, KEYB_HELP, &state))
	{
		e->inter_time = get_time();
		e->inter_state = (e->inter_state) ? false : true;
	}
	if (event_register(e, KEYB_P, &state))
		bmp_save(NULL, WIDTH, HEIGHT, (int *)e->img_string);
	return (state);
}
