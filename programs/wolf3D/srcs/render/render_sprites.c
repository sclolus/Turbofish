/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   render_sprites.c                                   :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: stoupin <marvin@42.fr>                     +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/07/06 12:23:44 by stoupin           #+#    #+#             */
/*   Updated: 2017/07/06 12:23:45 by stoupin          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include <stdlib.h>
#include "render.h"

static inline int	angle_to_pix(float angle)
{
	int x;

	x = (int)(tanf(angle) / tanf((float)VIEW_ANGLE / 2.f) * (WIDTH / 2));
	return (x);
}

t_sprite			**create_z_buffer_order(t_env *env)
{
	int			i;
	int			l;
	t_sprite	**tmp;

	if (!(tmp = (t_sprite **)malloc((env->n_sprites) * sizeof(t_sprite *))))
		exit(EXIT_FAILURE);
	i = 0;
	while (i < env->n_sprites)
	{
		env->sprites[i].angle0_x = atan2f(env->sprites[i].location.y -
			env->player.location.y, env->sprites[i].location.x -
			env->player.location.x) - env->player.angle;
		l = angle_to_pix(env->sprites[i].angle0_x) + WIDTH / 2;
		if (l < 0)
			l = 0;
		if (l >= WIDTH)
			l = WIDTH - 1;
		env->sprites[i].dist = dist(env->player.location,
								env->sprites[i].location) * env->cos_list[l];
		tmp[i] = &env->sprites[i];
		i++;
	}
	ft_merge_tab((void ***)&tmp, env->n_sprites, &m_cmp);
	return (tmp);
}

inline static void	loop(t_env *e, t_sprite_env *s, t_sprite **tmp)
{
	t_pix			pix;

	while (++s->c.x < s->c_max.x)
	{
		if (e->scene.columns[s->c.x].wall_h_dist <= (*tmp)->dist)
			continue ;
		s->c.y = ((s->c_topleft.y >= 0) ? s->c_topleft.y : 0) - 1;
		s->c_max.y = (s->c_bottomright.y < HEIGHT) ? s->c_bottomright.y :
															HEIGHT - 1;
		while (++s->c.y < s->c_max.y && e->scene.n_layer_sprite <
															WIDTH * HEIGHT)
		{
			s->c_tex.x = (float)(s->c.x - s->c_topleft.x) /
				(s->c_bottomright.x - s->c_topleft.x) *
				(e->scene.bmp_sprite[(*tmp)->type].dim.x - 2);
			s->c_tex.y = (float)(s->c.y - s->c_topleft.y) /
				(s->c_bottomright.y - s->c_topleft.y) *
				(e->scene.bmp_sprite[(*tmp)->type].dim.y - 2);
			pix = get_pix(&(e->scene.bmp_sprite[(*tmp)->type]),
								s->c_tex, (*tmp)->dist, e->inter_state);
			if (pix.c.a != 0xff)
				e->scene.scene[s->c.y * WIDTH + s->c.x] = pix;
		}
	}
}

inline static void	mass_init(t_env *e, t_sprite_env *s, t_sprite **tmp)
{
	s->angle_topleft.x = (*tmp)->angle0_x - atanf(.5f / (*tmp)->dist);
	s->angle_bottomright.x = (*tmp)->angle0_x + atanf(.5f / (*tmp)->dist);
	s->c_topleft.x = angle_to_pix(s->angle_topleft.x) + WIDTH / 2;
	s->c_bottomright.x = angle_to_pix(s->angle_bottomright.x) + WIDTH / 2;
	s->angle_topleft.y = atanf((e->sprite_height - e->player.height) /
															(*tmp)->dist);
	s->angle_bottomright.y = atanf(-e->player.height / (*tmp)->dist);
	s->c_topleft.y = HEIGHT / 2 - angle_to_pix(s->angle_topleft.y);
	s->c_bottomright.y = HEIGHT / 2 - angle_to_pix(s->angle_bottomright.y);
	s->c.x = ((s->c_topleft.x >= 0) ? s->c_topleft.x : 0) - 1;
	s->c_max.x = (s->c_bottomright.x < WIDTH) ? s->c_bottomright.x : WIDTH - 1;
}

void				render_sprites(t_env *e)
{
	t_sprite_env		s;
	t_sprite			**tmp;
	int					i;

	tmp = create_z_buffer_order(e);
	i = 0;
	e->sprite_height = 1.2;
	while (i < e->n_sprites)
	{
		if ((*tmp)->angle0_x < 0.f)
			(*tmp)->angle0_x += 2.f * PI;
		if (cosf((*tmp)->angle0_x) <= 0.)
		{
			tmp++;
			i++;
			continue ;
		}
		mass_init(e, &s, tmp);
		loop(e, &s, tmp);
		tmp++;
		i++;
	}
	free(tmp -= e->n_sprites);
}
