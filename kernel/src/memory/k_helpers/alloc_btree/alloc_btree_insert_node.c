
#include "alloc_btree_internal_header.h"

/*
** Recursive descent.
*/

static void	insert_child(
		struct s_node *parent,
		struct s_node *new,
		int (*cmpf)(struct s_node *, struct s_node *))
{
	if (cmpf(new, parent) < 0) {
		if (!parent->left) {
			parent->left = new;
			new->parent = parent;
		} else {
			insert_child(parent->left, new, cmpf);
		}
	} else {
		if (!parent->right) {
			parent->right = new;
			new->parent = parent;
		} else {
			insert_child(parent->right, new, cmpf);
		}
	}
}

struct s_node	*alloc_btree_internal_insert_node(
		struct s_node **root,
		struct s_node *new,
		int (*cmpf)(struct s_node *, struct s_node *))
{
	new->left = NULL;
	new->right = NULL;
	if (!(*root)) {
		*root = new;
		new->parent = NULL;
	}
	else {
		insert_child(*root, new, cmpf);
	}
	return (new);
}

struct s_node	*alloc_btree_insert_node(
		struct s_node **root,
		struct s_node *new,
		int (*cmpf)(struct s_node *, struct s_node *))
{
	if (root == NULL || new == NULL || cmpf == NULL || new->ptr_a == NULL)
		return (NULL);
	alloc_btree_internal_insert_node(root, new, cmpf);
	SET_BLACK(new);
	return (new);
}
