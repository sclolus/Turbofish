/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   debug_display_nodes.c                              :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2018/04/22 17:55:27 by bmickael          #+#    #+#             */
/*   Updated: 2018/04/22 18:11:56 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "main_headers.h"

static inline void	show_part(struct s_node_page *e, int i, int fd)
{
	(void)i;
	(void)e;
	(void)fd;
//	ft_fprintf(fd, "%.3i: ", i);
//	ft_fprintf(fd, "{red}%s{eoc}", e->node[i].parent == NULL ?
//			"ROOT " : "     ");

/*
	if (e->node[i].mask.s.node_type == INDEX)
		ft_fprintf(fd, "UNIQUE: index_pages_tree, %p", &e->node[i]);
	else if (e->node[i].mask.s.node_type == RECORD_ALLOCATED_LARGE)
		ft_fprintf(fd, "UNIQUE: big_page_record_tree");
	else if (e->node[i].mask.s.node_type == PARENT_RECORD_FREE_TINY)
		ft_fprintf(fd, "UNIQUE: global_tiny_space_tree");
	else if (e->node[i].mask.s.node_type == PARENT_RECORD_FREE_MEDIUM)
		ft_fprintf(fd, "UNIQUE: global_medium_space_tree");
	else if (e->node[i].mask.s.node_type == RECORD_ALLOCATED_TINY)
		ft_fprintf(fd, "        record_allocated_tiny");
	else if (e->node[i].mask.s.node_type == RECORD_ALLOCATED_MEDIUM)
		ft_fprintf(fd, "        record_allocated_medium");
	else if (e->node[i].mask.s.node_type == RECORD_FREE_TINY)
		ft_fprintf(fd, "        record_free_tiny");
	else if (e->node[i].mask.s.node_type == RECORD_FREE_MEDIUM)
		ft_fprintf(fd, "        record_free_medium");
	else
		ft_fprintf(fd, "{red}unknown node{eoc}");
	ft_fprintf(fd, "\n");
*/
}

void			debug_nodes(int fd)
{
	struct s_node_page	*e;
	int			i;

	(void)fd;
//	ft_fprintf(fd, "{red}__ALLOCATED_NODES__:{eoc}\n");
	e = ctx.node_pages_entry;
	while (e)
	{
		i = 0;
		while (i < e->primary_block.nb_node)
		{
			show_part(e, i, fd);
			i++;
		}
		e = e->primary_block.next;
//		ft_fprintf(fd, "\n");
	}
}
