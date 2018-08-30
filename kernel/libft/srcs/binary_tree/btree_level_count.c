/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   btree_level_count.c                                :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/03/22 17:45:19 by bmickael          #+#    #+#             */
/*   Updated: 2017/03/24 07:11:41 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "btree_internal_header.h"

static int	recurse_level_count(struct s_node *root, int n)
{
	int i;
	int j;

	i = n;
	j = n;
	if (root)
	{
		if (root->left)
			i = recurse_level_count(root->left, n + 1);
		if (root->right)
			j = recurse_level_count(root->right, n + 1);
	}
	return ((i > j) ? i : j);
}

int			btree_level_count(struct s_node *root)
{
	if (!root)
		return (0);
	return (recurse_level_count(root, 1));
}
