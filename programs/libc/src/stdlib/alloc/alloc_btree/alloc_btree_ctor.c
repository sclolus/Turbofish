
#include "alloc_btree_internal_header.h"

struct s_node	*alloc_btree_new(void)
{
	return (NULL);
}

/*
** Delete Constructor use suffix method.
*/

int		alloc_btree_delete(
		struct s_node *root,
		void (*deallocator)(void *))
{
	if (root) {
		if (deallocator == NULL)
			return (-EINVAL);
		if (root->left)
			alloc_btree_delete(root->left, deallocator);
		if (root->right)
			alloc_btree_delete(root->right, deallocator);
		deallocator(root);
	}
	return (0);
}

size_t		alloc_btree_get_node_size(void)
{
	return (sizeof(struct s_node));
}
