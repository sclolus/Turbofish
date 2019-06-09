
#include "alloc_btree_internal_header.h"

struct s_node	*alloc_btree_create_node(void *(*allocator)(size_t))
{
	struct s_node *new;

	if (allocator == NULL)
		return (NULL);
	new = (struct s_node *)allocator(sizeof(struct s_node));
	if (new == NULL)
		return (NULL);
	new->left = NULL;
	new->right = NULL;
	new->parent = NULL;
	return (new);
}

int		alloc_btree_destroy_node(
		struct s_node *node,
		void (*deallocator)(void *))
{
	if (node == NULL || deallocator == NULL)
		return (-EINVAL);
	deallocator(node);
	return (0);
}
