/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   btree_family_node.c                                :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/03/22 13:16:59 by bmickael          #+#    #+#             */
/*   Updated: 2017/03/22 13:34:46 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "btree_internal_header.h"

struct s_node	*btree_get_parent(struct s_node *n)
{
	return (n->parent);
}

/*
** Cannot have grandparent if no parent.
*/

struct s_node	*btree_get_grandparent(struct s_node *n)
{
	struct s_node *parent;

	parent = n->parent;
	if (parent)
		return (parent->parent);
	return (NULL);
}

/*
** Cannot have sibling if no parent.
*/

struct s_node	*btree_get_sibling(struct s_node *n)
{
	struct s_node *parent;

	parent = n->parent;
	if (parent)
		return ((n != parent->right) ? parent->right : parent->left);
	return (NULL);
}

/*
** Cannot have uncle if no grandparent.
*/

struct s_node	*btree_get_uncle(struct s_node *n)
{
	struct s_node *grandparent;

	grandparent = btree_get_grandparent(n);
	if (grandparent)
	{
		return ((n->parent != grandparent->right) ?
				grandparent->right : grandparent->left);
	}
	return (NULL);
}
