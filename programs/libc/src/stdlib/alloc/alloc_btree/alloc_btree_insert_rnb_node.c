
#include "alloc_btree_internal_header.h"

/*
** To add in a black and White tree:
** First: Apply a classic binary tree insertion
** By default, the new node is RED.
** Rule 1 -> If new node is root, color it to BLACK
** Rule 2 -> If his parent is black, don't do anything
** Rule 3 -> If his parent is red: Look the color of the uncle.
** It's decline in four cases:
** Finally, after changes, find the new root (is it's moved)
** And color it to BLACK
*/

struct s_node	*alloc_btree_insert_rnb_node(
		struct s_node **root,
		struct s_node *new,
		int (*cmpf)(struct s_node *, struct s_node *))
{
	struct s_node *clone;
	struct s_node *new_root;

	if (root == NULL || new == NULL || cmpf == NULL)
		return (NULL);
	clone = alloc_btree_internal_insert_node(root, new, cmpf);
	if (clone != new)
		return (NULL);
	SET_RED(new);
	apply_insert_strategy(new);
	new_root = new;
	while (new_root->parent != NULL)
		new_root = new_root->parent;
	*root = new_root;
	SET_BLACK((*root));
	return (new);
}
