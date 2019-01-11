
#include "alloc_btree_internal_header.h"

/*
** Logically, if the tree is okay, highest item is in far right sector.
*/

struct s_node	*alloc_btree_get_highest_node(struct s_node *node)
{
	if (node == NULL)
		return (NULL);
	while (node->right)
		node = node->right;
	return (node);
}

/*
** Logically, if the tree is okay, lowest item is in far left sector.
*/

struct s_node	*alloc_btree_get_lowest_node(struct s_node *node)
{
	if (node == NULL)
		return (NULL);
	while (node->left)
		node = node->left;
	return (node);
}

int		alloc_btree_is_last_node(struct s_node *node)
{
	if (node == NULL)
		return (-EINVAL);
	if (node->parent == NULL && node->left == NULL && node->right == NULL)
		return (1);
	return (0);
}
