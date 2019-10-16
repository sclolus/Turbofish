/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   verif_map_range.c                                  :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/07/05 06:13:25 by bmickael          #+#    #+#             */
/*   Updated: 2017/07/05 06:13:29 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "parse/internal_parse.h"
#include "core/wolf3d.h"

int				verif_texture_range(t_env *e, int n_floor,
									int n_wall, int n_sprite)
{
	t_list		*lst;
	char		*content;

	if (n_floor <= 0 || n_wall <= 0 || n_sprite <= 0)
		return (-1);
	lst = e->content->data;
	while (lst)
	{
		content = lst->content;
		while (*content)
		{
			if (content[0] >= '1' && content[0] <= '9' &&
				(content[0] - '0') > (n_floor - 1))
				return (-1);
			if (content[0] >= 'a' && content[0] <= 'z' &&
				(content[0] - 'a') > (n_wall - 1))
				return (-1);
			if (content[1] >= '0' && content[1] <= '9' &&
				(content[1] - '0') > (n_sprite - 1))
				return (-1);
			content += (content[2] != '\0') ? 3 : 2;
		}
		lst = lst->next;
	}
	return (0);
}
