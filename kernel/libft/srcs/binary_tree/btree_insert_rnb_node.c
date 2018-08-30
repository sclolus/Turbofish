/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   btree_insert_rns_node.c                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/03/22 14:13:04 by bmickael          #+#    #+#             */
/*   Updated: 2017/03/24 07:10:12 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "btree_internal_header.h"

/*
** To add in a black and White tree:
** First: Apply a classic binary tree insertion
** By default, the new node is RED.
** Rule 1 -> If new node is root, color it to BLACK
** Rule 2 -> If his parent is black, don't do anything
** Rule 3 -> If his parent is red: Look the color of the uncle.
** It's decline in four cases:
** Finally, after changes, find the new root (is it's moved)
** And color it to BLACK
*/

struct s_node	*btree_insert_rnb_node_by_content(
		struct s_node **root,
		void *content,
		int (*cmpf)(void *, void *),
		void *(*allocator)(size_t))
{
	struct s_node *new;
	struct s_node *new_root;

	if (root == NULL || cmpf == NULL || allocator == NULL)
		return (NULL);
	new = btree_internal_insert_node_content(root, content, cmpf, allocator);
	if (new == NULL)
		return (NULL);
	SET_RED(new);
	apply_insert_strategy(new);
	new_root = new;
	while (new_root->parent != NULL)
		new_root = new_root->parent;
	*root = new_root;
	SET_BLACK((*root));
	return (new);
}

struct s_node	*btree_insert_rnb_node(
		struct s_node **root,
		struct s_node *new,
		int (*cmpf)(void *, void *))
{
	struct s_node *clone;
	struct s_node *new_root;

	if (root == NULL || new == NULL || cmpf == NULL || new->content == NULL)
		return (NULL);
	clone = btree_internal_insert_node(root, new, cmpf);
	if (clone != new)
		return (NULL);
	SET_RED(new);
	apply_insert_strategy(new);
	new_root = new;
	while (new_root->parent != NULL)
		new_root = new_root->parent;
	*root = new_root;
	SET_BLACK((*root));
	return (new);
}
