
#include "alloc_btree_internal_header.h"

static struct s_node	*node_recursive_candidate(
			struct s_node *node,
			struct s_node *last_candidate,
			void *data_ref,
			int (*cmpf)(void *, struct s_node *))
{
	int diff;

	if (node == NULL)
		return (last_candidate);
	diff = cmpf(data_ref, node);
	if (diff == 0)
		return (node);
	if (diff < 0)
		return (node_recursive_candidate(
				node->left, node, data_ref, cmpf));
	return (node_recursive_candidate(
			node->right, last_candidate, data_ref, cmpf));
}

struct s_node		*alloc_btree_get_last_valid_node(
			struct s_node *root,
			void *data_ref,
			int (*cmpf)(void *, struct s_node *))
{
	if (root == NULL || cmpf == NULL)
		return (NULL);
	return (node_recursive_candidate(root, NULL, data_ref, cmpf));
}
