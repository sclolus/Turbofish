
#include "alloc_btree_internal_header.h"

static void	*alloc_btree_aligned_memcpy_four(
		void *restrict dst,
		const void *restrict src,
		size_t n)
{
	uint32_t *src1;
	uint32_t *dst1;

	if (src == dst)
		return ((void *)src);
	src1 = (uint32_t *)src;
	dst1 = (uint32_t *)dst;
	n >>= 2;
	while (n--)
		*dst1++ = *src1++;
	return (dst);
}

int		alloc_btree_memory_move(void *dest, struct s_node *src_node)
{
	struct s_node	*dest_node;
	int		parent_position;
	size_t		node_size;

	if (src_node == NULL || dest == NULL)
		return (-EINVAL);
	if (src_node->parent)
		parent_position = (src_node->parent->left == src_node) ? -1 : 1;
	else
		parent_position = 0;

	node_size = sizeof(struct s_node);
	alloc_btree_aligned_memcpy_four(dest, src_node, node_size);
	dest_node = (struct s_node *)dest;
	if (dest_node->left)
		dest_node->left->parent = dest_node;
	if (dest_node->right)
		dest_node->right->parent = dest_node;
	if (parent_position == -1)
		dest_node->parent->left = dest_node;
	else if (parent_position == 1)
		dest_node->parent->right = dest_node;
	return (0);
}
