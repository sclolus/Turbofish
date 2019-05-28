
#include "alloc_btree_internal_header.h"

/*
** See this video "Red-black trees in 3 minutes — Rotations" to understand.
** This below rotations are pure pointer redirection, none color changed.
** Use it for deletion.
*/

void	alloc_btree_rotate_left(struct s_node *node)
{
	struct s_node *right_child;

	right_child = node->right;
	right_child->parent = node->parent;
	if (node->parent) {
		if (node->parent->left == node)
			node->parent->left = right_child;
		else
			node->parent->right = right_child;
	}
	node->right = right_child->left;
	if (node->right)
		node->right->parent = node;
	right_child->left = node;
	node->parent = right_child;
}

void	alloc_btree_rotate_right(struct s_node *node)
{
	struct s_node *left_child;

	left_child = node->left;
	left_child->parent = node->parent;
	if (node->parent) {
		if (node->parent->left == node)
			node->parent->left = left_child;
		else
			node->parent->right = left_child;
	}
	node->left = left_child->right;
	if (node->left)
		node->left->parent = node;
	left_child->right = node;
	node->parent = left_child;
}
