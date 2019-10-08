/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   parse.h                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: stoupin <stoupin@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/07/05 04:33:29 by bmickael          #+#    #+#             */
/*   Updated: 2018/02/01 17:16:22 by stoupin          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#ifndef PARSE_H
# define PARSE_H

# include "libft.h"
# include "graphic_types.h"
# include "core/wolf3d.h"

typedef struct	s_sprite_info
{
	t_coord_f	location;
	int			type;
}				t_sprite_info;

int				load_map(t_env *e, char *filename);
int				get_player_location(t_env *e, t_coord_f *l, char c);
int				get_nbr_sprites(t_env *e);
t_sprite_info	*get_sprites(t_env *e, int n);
int				**get_map_struct(t_env *e, int *height, int *width);
void			free_map_content(t_map_content **content);
int				verif_texture_range(t_env *e, int n_floor,
									int n_wall, int n_sprite);

#endif
