/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   constructor.c                                      :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/07/05 04:32:26 by bmickael          #+#    #+#             */
/*   Updated: 2017/07/05 04:32:29 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include <stdlib.h>
#include <stdio.h>
#include <unistd.h>

#include "parse/internal_parse.h"

void	alloc_map_content(t_map_content **content)
{
	if (*content != NULL)
	{
		dprintf(STDERR_FILENO, "{green}parse:{eoc} content already allocated\n");
		exit(EXIT_FAILURE);
	}
	if (!(*content = (t_map_content *)calloc(1, sizeof(t_map_content))))
		exit(EXIT_FAILURE);
}

void	free_map_content(t_map_content **content)
{
	t_list		*lst;
	t_list		*next;

	if (*content == NULL)
	{
		dprintf(STDERR_FILENO, "{green}parse:{eoc} content wasn't allocated\n");
		exit(EXIT_FAILURE);
	}
	if ((*content)->data)
	{
		lst = (*content)->data;
		while (lst)
		{
			next = lst->next;
			if (lst->content)
				free(lst->content);
			free(lst);
			lst = next;
		}
	}
	free(*content);
	*content = NULL;
}
