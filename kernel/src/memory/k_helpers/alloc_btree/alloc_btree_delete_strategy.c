
#include "alloc_btree_internal_header.h"

/*
** The DB_BLACK stay and we found it a new sibling
*/

void		sibling_is_red(struct s_node **sibling, struct s_node **root)
{
	struct s_node *parent;

	parent = (*sibling)->parent;
	if (*sibling == parent->left) {
		alloc_btree_rotate_right(parent);
		SET_BLACK((*sibling));
		SET_RED((*sibling)->right);
		if (parent == *root)
			*root = *sibling;
		*sibling = parent->left;
	} else {
		alloc_btree_rotate_left(parent);
		SET_BLACK((*sibling));
		SET_RED((*sibling)->left);
		if (parent == *root)
			*root = *sibling;
		*sibling = parent->right;
	}
}

/*
** XXX Norm incomplete. We need to throw an exception if fail after if-else.
*/

void		minor_rotations_case(
		struct s_node *sibling,
		struct s_node **root)
{
	struct s_node *parent;

	parent = sibling->parent;
	if (sibling == parent->left && IS_BLACK(sibling->left)) {
		alloc_btree_rotate_left(sibling);
		SET_BLACK(sibling->parent);
		SET_RED(sibling);
		major_rotations_case(sibling->parent, root);
	} else if (sibling == parent->right && IS_BLACK(sibling->right)) {
		alloc_btree_rotate_right(sibling);
		SET_BLACK(sibling->parent);
		SET_RED(sibling);
		major_rotations_case(sibling->parent, root);
	}
}

void		major_rotations_case(
		struct s_node *sibling,
		struct s_node **root)
{
	struct s_node *parent;

	parent = sibling->parent;
	if (sibling == parent->left && IS_RED(sibling->left)) {
		alloc_btree_rotate_right(parent);
		if (IS_RED(sibling->left) && IS_RED(sibling->right))
			SET_RED(sibling);
		SET_BLACK(sibling->left);
		SET_BLACK(sibling->right);
	} else if (sibling == parent->right && IS_RED(sibling->right)) {
		alloc_btree_rotate_left(parent);
		if (IS_RED(sibling->left) && IS_RED(sibling->right))
			SET_RED(sibling);
		SET_BLACK(sibling->left);
		SET_BLACK(sibling->right);
	} else {
		minor_rotations_case(sibling, root);
	}

	if (parent == *root)
		*root = sibling;
}

static void	loop(
		struct s_node *trash,
		struct s_node **root,
		struct s_node *sibling)
{
	while (IS_DB_BLACK(trash))
		if (IS_BLACK(sibling)) {
			if (IS_RED(sibling->left) || IS_RED(sibling->right)) {
				major_rotations_case(sibling, root);
				SET_BLACK(trash);
			} else {
				SET_RED(sibling);
				SET_BLACK(trash);
				trash = sibling->parent;
				if (IS_RED(trash)) {
					SET_BLACK(trash);
				} else if (trash != *root) {
					SET_DB_BLACK(trash);
					if (trash->parent->left != trash)
						sibling = trash->parent->left;
					else
						sibling = trash->parent->right;
				}
			}
		} else {
			sibling_is_red(&sibling, root);
		}
}

void		apply_delete_strategy(
		struct s_node *trash,
		struct s_node **root,
		struct s_node *sibling)
{
	struct s_node *trash_child;

	if (IS_RED(trash)) {
		SET_BLACK(trash->parent);
	} else {
		if (trash->left || trash->right) {
			trash_child = (trash->left) ?
					trash->left : trash->right;
			if (IS_RED(trash_child))
				SET_BLACK(trash_child);
			else
				SET_DB_BLACK(trash);
		} else {
			SET_DB_BLACK(trash);
		}
	}
	loop(trash, root, sibling);
	SET_BLACK((*root));
	return ;
}
