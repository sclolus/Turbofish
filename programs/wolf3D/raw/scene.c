/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   scene.c                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/06/08 00:19:32 by bmickael          #+#    #+#             */
/*   Updated: 2017/06/08 00:19:35 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

static void		merge_layers(t_env *env)
{
	int		i;
	t_pix	*scene;

	scene = env->scene.scene;
	i = -1;
	while (++i < WIDTH * HEIGHT)
		scene[i].i = 0;
	rendering_layer_put(scene, &(env->scene.sky));
	rendering_layer_put(scene, &(env->scene.wall));
	rendering_layer_put(scene, &(env->scene.floor));
}

void			render_scene(t_env *env)
{
	int			x;
	t_coord_f	c_intersect;
	t_column	*c;

	x = -1;
	while (++x < WIDTH)
	{
		c = &(env->scene.columns[x]);
		c->angle_x = angle_on_screen(x - (WIDTH / 2)) + env->player.angle;
		find_wall(env, c->angle_x, &c_intersect, &(c->wall_x_tex));
		c->wall_h_dist = dist(env->player.location, c_intersect);
		c->wall_min_angle = atanf(-env->player.height / c->wall_h_dist);
		c->wall_max_angle = atanf((env->wall_height - env->player.height)
									/ c->wall_h_dist);
	}
	render_sky(env, &(env->scene.sky));
	render_wall(env, &(env->scene.wall));
	render_floor(env, &(env->scene.floor));
	merge_layers(env);
}
