/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   btree_apply_prefix.c                               :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/03/22 14:42:12 by bmickael          #+#    #+#             */
/*   Updated: 2017/03/22 16:57:47 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "btree_internal_header.h"

int		btree_apply_prefix(struct s_node *root, void (*applyf)(void *))
{
	if (applyf == NULL)
		return (-EINVAL);
	if (root)
	{
		applyf(root->content);
		if (root->left)
			btree_apply_prefix(root->left, applyf);
		if (root->right)
			btree_apply_prefix(root->right, applyf);
	}
	return (0);
}
