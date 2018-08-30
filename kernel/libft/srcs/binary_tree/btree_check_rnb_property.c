/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   btree_check_rnb_property.c                         :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/03/22 16:55:39 by bmickael          #+#    #+#             */
/*   Updated: 2017/03/24 02:50:53 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "btree_internal_header.h"

static void		btree_suffix_test_homogenetic_black(
		struct s_node *node,
		int level,
		enum e_check_result *homogenetic_black)
{
	static int black_level_count = -1;

	if (level == -1)
	{
		black_level_count = -1;
		level = 0;
	}
	if (node)
	{
		if (node->color == BLACK)
			level += 1;
		if (node->left)
			btree_suffix_test_homogenetic_black(node->left, level,
					homogenetic_black);
		if (node->right)
			btree_suffix_test_homogenetic_black(node->right, level,
					homogenetic_black);
		if (node->right == NULL && node->left == NULL)
		{
			if (black_level_count == -1)
				black_level_count = level;
			else if (black_level_count != level)
				*homogenetic_black = FAILED;
		}
	}
}

static void		btree_suffix_test_filiation(
		struct s_node *node,
		struct s_node *root,
		struct s_node *parent,
		enum e_check_result *filiation)
{
	if (node)
	{
		if (node == root && node->parent != NULL)
			*filiation = FAILED;
		if (node->left)
			btree_suffix_test_filiation(node->left, root, node, filiation);
		if (node->right)
			btree_suffix_test_filiation(node->right, root, node, filiation);
		if (node->parent != parent)
			*filiation = FAILED;
	}
}

static int		btree_suffix_count_nodes(struct s_node *node)
{
	int n;

	n = 0;
	if (node)
	{
		if (node->left)
			n += btree_suffix_count_nodes(node->left);
		if (node->right)
			n += btree_suffix_count_nodes(node->right);
		return (n + 1);
	}
	return (0);
}

static void		btree_suffix_check_red_black_interlacement(
		struct s_node *node,
		struct s_node *root,
		enum e_check_result *rnb_interlacement)
{
	if (node)
	{
		if (node->color == RED && node->parent && node->parent->color != BLACK)
			*rnb_interlacement = FAILED;
		if (node->color == RED && node->left && node->left->color != BLACK)
			*rnb_interlacement = FAILED;
		if (node->color == RED && node->right && node->right->color != BLACK)
			*rnb_interlacement = FAILED;
		if (node->left)
			btree_suffix_check_red_black_interlacement(node->left,
					root, rnb_interlacement);
		if (node->right)
			btree_suffix_check_red_black_interlacement(node->right,
					root, rnb_interlacement);
	}
}

int				btree_check_rnb_property(struct s_node *root,
		struct s_rnb_tree_checker_result *result)
{
	if (result == NULL)
		return (-EINVAL);
	result->root_is_black = OK;
	result->homogenetic_black = OK;
	result->filiation = OK;
	result->rnb_interlacement = OK;
	result->nb_levels = 0;
	result->nb_nodes = 0;
	if (root == NULL)
		return (0);
	if (root->color != BLACK)
		result->root_is_black = FAILED;
	btree_suffix_test_filiation(root, root, NULL, &result->filiation);
	btree_suffix_test_homogenetic_black(root, -1, &result->homogenetic_black);
	btree_suffix_check_red_black_interlacement(
		root,
		root,
		&result->rnb_interlacement);
	result->nb_levels = btree_level_count(root);
	result->nb_nodes = btree_suffix_count_nodes(root);
	return (0);
}
