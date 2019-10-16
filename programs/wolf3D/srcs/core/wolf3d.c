/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   wolf3d.c                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: stoupin <stoupin@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/06/03 15:42:11 by bmickael          #+#    #+#             */
/*   Updated: 2018/02/01 14:25:00 by stoupin          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include <math.h>
#include <stdlib.h>
#include <stdbool.h>

#include "core/wolf3d.h"
#include "parse/parse.h"
#include "overlay/overlay.h"
#include "render/render.h"

static int move(t_env *e)
{
	t_pix pix;

	animate_sprites(e);
	common_action(e);
	move_player(e);
	render_scene(e);
	if (e->display_minimap)
		draw_minimap(e);
	scene_to_win(e);
	pix.i = 0xff0000;
	draw_box((t_coord_i){WIDTH / 2 - 10, HEIGHT / 2 - 10},
	(t_coord_i){WIDTH / 2 + 10, HEIGHT / 2 + 10}, pix, e->scene.scene);
#ifdef GNU
	mlx_put_image_to_window(e->mlx, e->win, e->image, 0, 0);
#endif
	if (e->inter_time)
		interpolate_switch(e, get_time());
	eval_fps(e);
#ifndef GNU
	mlx_put_image_to_window(e->mlx, e->win, e->image, 0, 0);
#endif
	return (0);
}

static inline float angle_on_screen(int x)
{
	float angle;

	angle = atanf((float)x / (WIDTH / 2) * tanf((float)VIEW_ANGLE / 2.f));
	return (angle);
}

static void init_all(t_env *e)
{
	int i;

	create_mlx_image(e);

	init_sky(e, "images/astro.bmp");
	init_floor(e, (char*[]) {
			"images/parquet.bmp",
			"images/seamless_carpet.bmp",
			"images/Brick2.bmp"
	}, N_FLOOR_BMP);
	init_walls(e, (char*[]) {
			"images/4murs.bmp",
			"images/4murs2.bmp",
			"images/pig.bmp",
			"images/aperture.bmp"
	}, N_WALL_BMP);
	init_sprites(e, (char*[]) {
			"images/pig_2.bmp",
			"images/dog.bmp",
			"images/sadirac.bmp"
	}, N_SPRITE_BMP);

	init_scene(e);
	i = 0;
	while (i < HEIGHT) {
		e->angle_y[i] = angle_on_screen((HEIGHT / 2) - i);
		e->dist_floor[i] = e->player.height / tanf(-e->angle_y[i]);
		e->atan_list[i] = tanf(e->angle_y[i]);
		i++;
	}
	i = -1;
	while (++i < WIDTH) {
		e->angle_x[i] = angle_on_screen(i - (WIDTH / 2));
		e->cos_list[i] = cosf(e->angle_x[i]);
	}
}

static int get_parse(t_env *e, char *filename)
{
	t_sprite_info *s_l;
	int i;

	if (load_map(e, filename) != 0)
		return (err_msg("bad file"));
	if (get_player_location(e, &e->player.location, '%') != 0)
		return (err_msg("no player in the map !"));
	printf("location_player.x = %i, location_player.y = %i\n",
					(int)e->player.location.x, (int)e->player.location.y);
	e->n_sprites = get_nbr_sprites(e);
	s_l = get_sprites(e, e->n_sprites);
	if (!(e->sprites = (t_sprite*)malloc(sizeof(t_sprite) * e->n_sprites)))
		exit(EXIT_FAILURE);
	i = -1;
	while (++i < e->n_sprites) {
		printf("sprite type %i: x = %i && y = %i\n", s_l->type,
							(int)s_l->location.x, (int)s_l->location.y);
		e->sprites[i].location = s_l->location;
		e->sprites[i].type = (s_l++)->type;
	}
	printf("verif = %s\n", verif_texture_range(e, 3, 4, 3) ? "KO" : "OK");
	e->map_tiles = (t_tile **)get_map_struct(e, &e->map.size.y, &e->map.size.x);
	free_map_content(&(e->content));
	return (0);
}

int main(int argc, char **argv)
{
	t_env env;

	bzero(&env, sizeof(t_env));
	if (argc != 2)
		return (err_usage(argv[0]));
	if ((init_mlx(&env, "Wolf3D -^^,--,~", WIDTH * NOSTALGIA_FACTOR, HEIGHT * NOSTALGIA_FACTOR)))
		return (err_msg("Error during initialisation"));
	if (get_parse(&env, argv[1]))
		return (-1);
	view_map(env.map_tiles, env.map.size.x, env.map.size.y);
	init_sprite_ai(&env);
	env.wall_height = 3.f;
	env.sprite_height = 2.5f;
	env.player.angle = 6.f / 4 * PI;
	env.player.height = 2.f;
	env.display_minimap = true;
	env.inter_state = true;
	init_all(&env);
#ifdef GNU
	mlx_hook(env.win, X11_KEY_RELEASE, KEYRELEASEMASK, &mlx_key_release, &env);
	mlx_hook(env.win, X11_KEY_PRESS, KEYPRESSMASK, &mlx_key_press, &env);
	mlx_hook(env.win, X11_DESTROY_NOTIFY, 0xFF, &exit_mlx, &env);
#else
	mlx_hook(env.win, TURBOFISH_KEY_RELEASE, KEYRELEASEMASK, &mlx_key_release, &env);
	mlx_hook(env.win, TURBOFISH_KEY_PRESS, KEYPRESSMASK, &mlx_key_press, &env);
#endif
	mlx_loop_hook(env.mlx, &move, &env);
	mlx_loop(env.mlx);
	return (0);
}
