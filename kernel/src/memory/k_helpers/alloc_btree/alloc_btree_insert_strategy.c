/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   alloc_btree_insert_strategy.c                      :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/03/22 16:55:39 by bmickael          #+#    #+#             */
/*   Updated: 2017/03/24 02:50:53 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "alloc_btree_internal_header.h"

static void		do_rotation(struct s_node *new, struct s_node *parent)
{
	struct s_node *grandparent;

	grandparent = parent->parent;
	if (parent == grandparent->right && new == parent->left)
	{
		alloc_btree_rotate_right(parent);
		alloc_btree_rotate_left(grandparent);
		SET_BLACK(new);
		SET_RED(grandparent);
		return ;
	}
	else if (parent == grandparent->left && new == parent->right)
	{
		alloc_btree_rotate_left(parent);
		alloc_btree_rotate_right(grandparent);
		SET_BLACK(new);
		SET_RED(grandparent);
		return ;
	}
	if (new == parent->left)
		alloc_btree_rotate_right(grandparent);
	else
		alloc_btree_rotate_left(grandparent);
	SET_BLACK(parent);
	SET_RED(grandparent);
}

void			apply_insert_strategy(struct s_node *new)
{
	struct s_node *uncle;
	struct s_node *parent;

	parent = new->parent;
	if (parent == NULL)
	{
		SET_BLACK(new);
		return ;
	}
	if (IS_BLACK(parent))
		return ;
	uncle = alloc_btree_get_uncle(new);
	if (IS_BLACK(uncle))
		do_rotation(new, parent);
	else
	{
		SET_BLACK(parent);
		SET_BLACK(uncle);
		SET_RED(parent->parent);
		apply_insert_strategy(parent->parent);
	}
}
