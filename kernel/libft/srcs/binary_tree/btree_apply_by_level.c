/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   btree_apply_by_level.c                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/03/24 01:42:50 by bmickael          #+#    #+#             */
/*   Updated: 2017/03/24 02:37:21 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "btree_internal_header.h"

static int		p_recurse_level_count(struct s_node *root, int n)
{
	int i;
	int j;

	i = n;
	j = n;
	if (root)
	{
		if (root->left)
			i = p_recurse_level_count(root->left, n + 1);
		if (root->right)
			j = p_recurse_level_count(root->right, n + 1);
	}
	return ((i > j) ? i : j);
}

static void		rec_level(struct s_node *root, void (*applyf)(void *content,
		int current_level, int first_elem), int lvl, int *cap)
{
	if (lvl == *cap || lvl == -(*cap))
	{
		if (*cap >= 0)
		{
			applyf(root->content, lvl, 1);
			*cap = -(*cap);
		}
		else
			applyf(root->content, lvl, 0);
		return ;
	}
	if (root->left)
		rec_level(root->left, applyf, lvl + 1, cap);
	if (root->right)
		rec_level(root->right, applyf, lvl + 1, cap);
}

int				btree_apply_by_level(struct s_node *root,
		void (*applyf)(void *content, int current_level, int first_elem))
{
	int deep;
	int i;
	int cap;

	if (applyf == NULL)
		return (-EINVAL);
	if (root == NULL)
		return (0);
	deep = p_recurse_level_count(root, 0);
	i = 0;
	while (i <= deep)
	{
		cap = i;
		rec_level(root, applyf, 0, &cap);
		i++;
	}
	return (0);
}
