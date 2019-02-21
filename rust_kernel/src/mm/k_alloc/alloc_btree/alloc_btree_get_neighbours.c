
#include "alloc_btree_internal_header.h"

struct s_node	*alloc_btree_get_next_neighbours_node(struct s_node *node)
{
	if (node == NULL)
		return (NULL);
	if (node->right)
		return (alloc_btree_get_lowest_node(node->right));
	if (node->parent)
	{
		if (node->parent->left == node)
			return (node->parent);
		else
		{
			while (node->parent && node == node->parent->right)
				node = node->parent;
			return (node->parent);
		}
	}
	return (NULL);
}

struct s_node	*alloc_btree_get_prev_neighbours_node(struct s_node *node)
{
	if (node == NULL)
		return (NULL);
	if (node->left)
		return (alloc_btree_get_highest_node(node->left));
	if (node->parent)
	{
		if (node->parent->right == node)
			return (node->parent);
		else
		{
			while (node->parent && node == node->parent->left)
				node = node->parent;
			return (node->parent);
		}
	}
	return (NULL);
}
