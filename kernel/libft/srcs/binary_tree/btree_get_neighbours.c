/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   btree_getneighbours.c                              :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/03/22 13:16:59 by bmickael          #+#    #+#             */
/*   Updated: 2017/03/22 13:34:46 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "btree_internal_header.h"

struct s_node			*btree_get_next_neighbours_node(struct s_node *node)
{
	if (node == NULL)
		return (NULL);
	if (node->right)
		return (btree_get_lowest_node(node->right));
	if (node->parent)
	{
		if (node->parent->left == node)
			return (node->parent);
		else
		{
			while (node->parent && node == node->parent->right)
				node = node->parent;
			return (node->parent);
		}
	}
	return (NULL);
}

struct s_node			*btree_get_prev_neighbours_node(struct s_node *node)
{
	if (node == NULL)
		return (NULL);
	if (node->left)
		return (btree_get_highest_node(node->left));
	if (node->parent)
	{
		if (node->parent->right == node)
			return (node->parent);
		else
		{
			while (node->parent && node == node->parent->left)
				node = node->parent;
			return (node->parent);
		}
	}
	return (NULL);
}

void					*btree_get_next_neighbours_content(struct s_node *node)
{
	struct s_node *res_node;

	res_node = btree_get_next_neighbours_node(node);
	if (res_node != NULL)
		return (res_node->content);
	return (NULL);
}

void					*btree_get_prev_neighbours_content(struct s_node *node)
{
	struct s_node *res_node;

	res_node = btree_get_prev_neighbours_node(node);
	if (res_node != NULL)
		return (res_node->content);
	return (NULL);
}
