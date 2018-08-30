/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   btree_atomics_op.c                                 :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/03/22 13:38:18 by bmickael          #+#    #+#             */
/*   Updated: 2017/03/24 07:12:30 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "btree_internal_header.h"

struct s_node	*btree_create_node(void *content, void *(*allocator)(size_t))
{
	struct s_node *new;

	if (allocator == NULL)
		return (NULL);
	new = (struct s_node *)allocator(sizeof(struct s_node));
	if (new == NULL)
		return (NULL);
	new->left = NULL;
	new->right = NULL;
	new->parent = NULL;
	new->content = content;
	return (new);
}

int				btree_destoy_node(
		struct s_node *node,
		void (*deallocator)(void *))
{
	if (node == NULL || deallocator == NULL)
		return (-EINVAL);
	deallocator(node);
	return (0);
}

int				btree_attach_content(
		struct s_node *node,
		void *content,
		void (*deallocator)(void *))
{
	if (node == NULL)
		return (-EINVAL);
	if (deallocator)
		deallocator(node->content);
	node->content = content;
	return (0);
}

void			*btree_get_node_content(struct s_node *node)
{
	if (node)
		return (node->content);
	return (NULL);
}

int				btree_delete_node_content(
		struct s_node *node,
		void (*deallocator)(void *))
{
	if (node && deallocator && node->content)
	{
		deallocator(node->content);
		node->content = NULL;
		return (0);
	}
	return (-EINVAL);
}
