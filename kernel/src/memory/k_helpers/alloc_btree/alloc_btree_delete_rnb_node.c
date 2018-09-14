
#include "alloc_btree_internal_header.h"

int				alloc_btree_delete_rnb_node_by_content(
				struct s_node **root,
				void *content,
				int (*cmpf)(void *, struct s_node *),
				void (*deallocator)(void *))
{
	struct s_node *node_to_trash;
	struct s_node *sibling;

	sibling = NULL;
	if (root == NULL || cmpf == NULL || deallocator == NULL)
		return (-EINVAL);
	node_to_trash = alloc_btree_get_node_by_content(*root, content, cmpf);
	if (node_to_trash == NULL)
		return (-1);
	node_to_trash = alloc_btree_internal_trash_node(node_to_trash, root,
			&sibling);
	if (node_to_trash->parent != NULL)
		apply_delete_strategy(node_to_trash, root, sibling);
	else if (*root)
		SET_BLACK((*root));
	deallocator(node_to_trash);
	return (0);
}

int				alloc_btree_delete_rnb_node(
				struct s_node **root,
				struct s_node *node,
				void (*deallocator)(void *))
{
	struct s_node	*node_to_trash;
	struct s_node	*sibling;
	int				ret;

	sibling = NULL;
	if (root == NULL || node == NULL)
		return (-EINVAL);
	node_to_trash = alloc_btree_internal_trash_node(node, root, &sibling);
	if (node_to_trash->parent != NULL)
		apply_delete_strategy(node_to_trash, root, sibling);
	else if (*root)
		SET_BLACK((*root));
	ret = (*root == NULL) ? 0 : 1;
	if (deallocator)
		deallocator(node_to_trash);
	return (ret);
}
