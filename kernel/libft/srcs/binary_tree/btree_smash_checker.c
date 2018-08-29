/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   btree_smash_checker.c                              :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/03/22 16:55:39 by bmickael          #+#    #+#             */
/*   Updated: 2018/05/04 18:29:57 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "btree_internal_header.h"

static struct s_node	*simulate_insert_child(
		struct s_node *parent,
		void *content,
		int (*cmpf)(void *, void *))
{
	int diff;

	if ((diff = cmpf(content, parent->content)) < 0)
	{
		if (!parent->left)
			return (parent);
		else
			return (simulate_insert_child(parent->left, content, cmpf));
	}
	else if (diff > 0)
	{
		if (!parent->right)
			return (parent);
		else
			return (simulate_insert_child(parent->right, content, cmpf));
	}
	return (NULL);
}

static struct s_node	*no_root_case(
		struct s_node **root,
		void *content,
		void *(*allocator)(size_t))
{
	struct s_node	*new;

	if (!(new = (struct s_node *)allocator(sizeof(struct s_node))))
		return (NULL);
	new->left = NULL;
	new->right = NULL;
	new->content = content;
	*root = new;
	new->parent = NULL;
	return (new);
}

static struct s_node	*btree_collision_insert_node_content(
		struct s_node **root,
		void *content,
		int (*cmpf)(void *, void *),
		void *(*allocator)(size_t))
{
	struct s_node	*parent;
	struct s_node	*new;

	if (!(*root))
		return (no_root_case(root, content, allocator));
	parent = simulate_insert_child(*root, content, cmpf);
	if (parent == NULL)
		return (NULL);
	if (!(new = (struct s_node *)allocator(sizeof(struct s_node))))
		return (NULL);
	new->left = NULL;
	new->right = NULL;
	new->content = content;
	if (cmpf(content, parent->content) < 0)
		parent->left = new;
	else
		parent->right = new;
	new->parent = parent;
	return (new);
}

struct s_node			*btree_smash_checker(
		struct s_node **root,
		void *content,
		int (*cmpf)(void *, void *),
		void *(*allocator)(size_t))
{
	struct s_node *new;
	struct s_node *new_root;

	if (root == NULL || cmpf == NULL || allocator == NULL)
		return (NULL);
	new = btree_collision_insert_node_content(root, content, cmpf, allocator);
	if (new == NULL)
		return (NULL);
	new->color = RED;
	apply_insert_strategy(new);
	new_root = new;
	while (new_root->parent != NULL)
		new_root = new_root->parent;
	*root = new_root;
	(*root)->color = BLACK;
	return (new);
}
