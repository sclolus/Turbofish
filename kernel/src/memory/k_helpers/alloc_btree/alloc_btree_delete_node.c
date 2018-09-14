
#include "alloc_btree_internal_header.h"

/*
** This function return node will be trashed
*/

static struct s_node	*two_childs_case(
			struct s_node *node,
			struct s_node **sibling)
{
	struct s_node	*high_form_left;
	struct s_node	*child;
	struct s_node	*parent;

	high_form_left = alloc_btree_get_highest_node(node->left);
	if (sibling)
		*sibling = alloc_btree_get_sibling(high_form_left);
	alloc_btree_swap_data(node, high_form_left);
	parent = high_form_left->parent;
	child = high_form_left->left;
	if (parent != node)
		parent->right = child;
	else
		parent->left = child;
	if (child)
		child->parent = parent;
	return (high_form_left);
}

/*
** This function return node will be trashed
*/

static struct s_node	*one_child_case(
			struct s_node *node,
			struct s_node **root,
			struct s_node **sibling)
{
	struct s_node	*child;

	if (sibling)
		*sibling = alloc_btree_get_sibling(node);
	child = (node->left != NULL) ? node->left : node->right;
	child->parent = node->parent;
	if (node == *root) {
		*root = child;
	} else {
		if (child->parent->left == node)
			child->parent->left = child;
		else
			child->parent->right = child;
	}
	return (node);
}

/*
** This function return node will be trashed
** The first case is two child case.
** The second is implicitely one child case.
** And the third is implicitely no child case.
*/

struct s_node		*alloc_btree_internal_trash_node(
			struct s_node *node,
			struct s_node **root,
			struct s_node **sibling)
{
	struct s_node *parent;

	if (node->left != NULL && node->right != NULL)
		return (two_childs_case(node, sibling));
	else if (node->left != NULL || node->right != NULL)
		return (one_child_case(node, root, sibling));
	if (sibling)
		*sibling = alloc_btree_get_sibling(node);
	parent = node->parent;
	if (parent == NULL) {
		*root = NULL;
		return (node);
	} else if (parent->right == node) {
		parent->right = NULL;
	} else {
		parent->left = NULL;
	}
	return (node);
}

/*
** Example of call:
** int ret = tree_delete_node(&tree, ptr, &intcmp, &free);
*/

int			alloc_btree_delete_node_by_content(
			struct s_node **root,
			void *content,
			int (*cmpf)(void *, struct s_node *),
			void (*deallocator)(void *))
{
	struct s_node *node_to_trash;

	if (root == NULL || cmpf == NULL || deallocator == NULL)
		return (-EINVAL);
	node_to_trash = alloc_btree_get_node_by_content(*root, content, cmpf);
	if (node_to_trash) {
		node_to_trash = alloc_btree_internal_trash_node(
				node_to_trash, root, NULL);
		deallocator(node_to_trash);
	} else {
		return (-1);
	}
	return (0);
}

int			alloc_btree_delete_node(
			struct s_node **root,
			struct s_node *node,
			void (*deallocator)(void *))
{
	struct s_node *node_to_trash;

	if (root == NULL || node == NULL || deallocator == NULL)
		return (-EINVAL);
	node_to_trash = alloc_btree_internal_trash_node(node, root, NULL);
	deallocator(node_to_trash);
	return (0);
}
