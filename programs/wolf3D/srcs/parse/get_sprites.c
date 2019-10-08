/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   get_sprites.c                                      :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/07/05 04:33:03 by bmickael          #+#    #+#             */
/*   Updated: 2017/07/05 04:33:04 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include <stdlib.h>
#include "graphic_types.h"
#include "parse/internal_parse.h"
#include "parse/parse.h"

int						get_nbr_sprites(t_env *e)
{
	t_list	*tmp;
	char	*content;
	int		n;

	n = 0;
	tmp = e->content->data;
	while (tmp)
	{
		content = tmp->content;
		while (*content)
		{
			if ((content[0] == '_' || (content[0] >= '0' && content[0] <= '9'))
				&& content[1] >= '0' && content[1] <= '9')
				n++;
			content += (content[2] != '\0') ? 3 : 2;
		}
		tmp = tmp->next;
	}
	return (n);
}

static void				fill_sprite_list(t_sprite_info *s_info, t_list *lst)
{
	t_coord_f		l;
	int				i;
	char			*content;

	i = 0;
	l.y = 0;
	while (lst)
	{
		content = lst->content;
		l.x = 0;
		while (*content)
		{
			if ((content[0] == '_' || (content[0] >= '0' && content[0] <= '9'))
				&& content[1] >= '0' && content[1] <= '9')
			{
				s_info[i].location = l;
				s_info[i].type = content[1] - '0';
				i++;
			}
			content += (content[2] != '\0') ? 3 : 2;
			l.x += 1;
		}
		l.y += 1;
		lst = lst->next;
	}
}

t_sprite_info			*get_sprites(t_env *e, int n)
{
	t_sprite_info	*s_info;

	if (!(s_info = (t_sprite_info *)malloc(n * sizeof(t_sprite_info))))
		exit(EXIT_FAILURE);
	fill_sprite_list(s_info, e->content->data);
	return (s_info);
}
