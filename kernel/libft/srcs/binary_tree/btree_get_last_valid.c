/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   btree_get_last_valid.c                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/03/22 13:16:59 by bmickael          #+#    #+#             */
/*   Updated: 2017/03/22 13:34:46 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "btree_internal_header.h"

static struct s_node	*node_recursive_candidate(
		struct s_node *node,
		struct s_node *last_candidate,
		void *data_ref,
		int (*cmpf)(void *, void *))
{
	int diff;

	if (node == NULL)
		return (last_candidate);
	diff = cmpf(data_ref, node->content);
	if (diff == 0)
		return (node);
	if (diff < 0)
		return (node_recursive_candidate(node->left, node, data_ref, cmpf));
	return (node_recursive_candidate(
			node->right, last_candidate, data_ref, cmpf));
}

struct s_node			*btree_get_last_valid_node(
		struct s_node *root,
		void *data_ref,
		int (*cmpf)(void *, void *))
{
	if (root == NULL || cmpf == NULL)
		return (NULL);
	return (node_recursive_candidate(root, NULL, data_ref, cmpf));
}

void					*btree_get_last_valid_content(
		struct s_node *root,
		void *data_ref,
		int (*cmpf)(void *, void *))
{
	struct s_node *node;

	if (root == NULL || cmpf == NULL)
		return (NULL);
	node = node_recursive_candidate(root, NULL, data_ref, cmpf);
	if (node != NULL)
		return (node->content);
	return (NULL);
}
