/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   alloc_btree_apply_infix.c                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/03/22 16:51:45 by bmickael          #+#    #+#             */
/*   Updated: 2017/03/24 02:50:18 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "alloc_btree_internal_header.h"

int		alloc_btree_apply_infix(
		struct s_node *root,
		void (*applyf)(struct s_node *node))
{
	if (applyf == NULL)
		return (-EINVAL);
	if (root) {
		if (root->left)
			alloc_btree_apply_infix(root->left, applyf);
		applyf(root);
		if (root->right)
			alloc_btree_apply_infix(root->right, applyf);
	}
	return (0);
}
