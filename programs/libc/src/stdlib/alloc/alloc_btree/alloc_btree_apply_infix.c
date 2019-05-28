
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
