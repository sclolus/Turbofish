/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   btree_conditional_insert.c                         :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/03/22 16:55:39 by bmickael          #+#    #+#             */
/*   Updated: 2017/03/24 02:50:53 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "btree_internal_header.h"

static struct s_node	*simulate_insert_node(
		struct s_node *node,
		void *content,
		int (*cmpf)(void *, void *),
		enum e_node_register *reg)
{
	int diff;

	if ((diff = cmpf(content, node->content)) < 0)
	{
		if (node->left == NULL)
			return (node);
		else
			return (simulate_insert_node(
					node->left, content, cmpf, reg));
	}
	else if (diff > 0)
	{
		if (node->right == NULL)
			return (node);
		else
			return (simulate_insert_node(
					node->right, content, cmpf, reg));
	}
	*reg = NODE_ALREADY_PRESENT;
	return (node);
}

struct s_node			*btree_conditional_insert(
		struct s_node **root,
		void *content,
		int (*cmpf)(void *, void *),
		void *(*allocator)(size_t))
{
	struct s_node			*new;
	struct s_node			*parent;
	enum e_node_register	reg;

	if (root == NULL || cmpf == NULL || content == NULL || allocator == NULL)
		return (NULL);
	if (*root == NULL)
	{
		if ((new = btree_create_node(content, allocator)) == NULL)
			return (NULL);
		*root = new;
		return (new);
	}
	reg = NODE_NEW;
	parent = simulate_insert_node(*root, content, cmpf, &reg);
	if (reg == NODE_ALREADY_PRESENT)
		return (parent);
	if ((new = btree_create_node(content, allocator)) == NULL)
		return (NULL);
	else if (cmpf(content, parent->content) < 0)
		parent->left = new;
	else
		parent->right = new;
	new->parent = parent;
	return (new);
}
