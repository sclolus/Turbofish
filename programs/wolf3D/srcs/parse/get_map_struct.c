/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   get_map_struct.c                                   :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/07/05 04:32:35 by bmickael          #+#    #+#             */
/*   Updated: 2017/07/05 04:32:38 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include <stdlib.h>
#include "parse/internal_parse.h"
#include "core/wolf3d.h"

static int			**allocate_tab(int *height, int *width)
{
	int				**tab;
	int				i;

	if (!(tab = (int **)calloc(1, *height * sizeof(int *))))
		exit(EXIT_FAILURE);
	i = 0;
	while (i < *height)
	{
		if (!(tab[i] = (int *)calloc(1, *width * sizeof(int))))
			exit(EXIT_FAILURE);
		i++;
	}
	return (tab);
}

static void			fill_border(int **tab, int height, int width)
{
	int i;
	int j;

	i = 0;
	while (i < height)
	{
		tab[i][0] = 50;
		tab[i][width - 1] = 50;
		i++;
	}
	j = 0;
	while (j < width)
	{
		tab[0][j] = 50;
		tab[height - 1][j] = 50;
		j++;
	}
}

static void			fill_values(t_list *lst, int **tab)
{
	int				i;
	int				j;
	char			*line;

	i = 1;
	while (lst)
	{
		j = 1;
		line = lst->content;
		while (*line)
		{
			if (*line == '_')
				tab[i][j] = 0;
			if (*line >= '1' && *line <= '9')
				tab[i][j] = *line - '0';
			if (*line >= 'a' && *line <= 'z')
				tab[i][j] = *line - 'a' + 50;
			line += (line[2] != '\0') ? 3 : 2;
			j++;
		}
		i++;
		lst = lst->next;
	}
}

int					**get_map_struct(t_env *e, int *height, int *width)
{
	t_map_content	*content;
	int				**tab;

	content = e->content;
	*height = content->height + 2;
	*width = content->width + 2;
	tab = allocate_tab(height, width);
	fill_border(tab, *height, *width);
	fill_values(content->data, tab);
	return (tab);
}
