/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   btree_check_binary_tree.c                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/03/22 16:51:45 by bmickael          #+#    #+#             */
/*   Updated: 2017/03/24 02:50:18 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "btree_internal_header.h"

static int	apply_infix_ret(struct s_node *root, int (*applyf)(void *))
{
	int ret;

	ret = 0;
	if (root)
	{
		if (root->left)
			ret |= apply_infix_ret(root->left, applyf);
		ret |= applyf(root->content);
		if (root->right)
			ret |= apply_infix_ret(root->right, applyf);
	}
	return (ret);
}

int			btree_check_binary_tree(struct s_node *root, int (*applyf)(void *))
{
	if (applyf == NULL)
		return (-EINVAL);
	return (apply_infix_ret(root, applyf));
}
