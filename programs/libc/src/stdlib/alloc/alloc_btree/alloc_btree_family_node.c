
#include "alloc_btree_internal_header.h"

struct s_node	*alloc_btree_get_parent(struct s_node *n)
{
	return (n->parent);
}

/*
** Cannot have grandparent if no parent.
*/

struct s_node	*alloc_btree_get_grandparent(struct s_node *n)
{
	struct s_node *parent;

	parent = n->parent;
	if (parent)
		return (parent->parent);
	return (NULL);
}

/*
** Cannot have sibling if no parent.
*/

struct s_node	*alloc_btree_get_sibling(struct s_node *n)
{
	struct s_node *parent;

	parent = n->parent;
	if (parent)
		return ((n != parent->right) ? parent->right : parent->left);
	return (NULL);
}

/*
** Cannot have uncle if no grandparent.
*/

struct s_node	*alloc_btree_get_uncle(struct s_node *n)
{
	struct s_node *grandparent;

	grandparent = alloc_btree_get_grandparent(n);
	if (grandparent) {
		return ((n->parent != grandparent->right) ?
				grandparent->right : grandparent->left);
	}
	return (NULL);
}
