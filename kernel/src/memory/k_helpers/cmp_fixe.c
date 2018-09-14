
#include "main_headers.h"

int	cmp_addr_to_node_addr(void *addr, struct s_node *node)
{
	if (addr < node->ptr_a)
		return (-1);
	if (addr > node->ptr_a)
		return (1);
	return (0);
}

int	cmp_node_addr_to_node_addr(struct s_node *node_a, struct s_node *node_b)
{
	if (node_a->ptr_a < node_b->ptr_a)
		return (-1);
	if (node_a->ptr_a > node_b->ptr_a)
		return (1);
	return (0);
}

int	cmp_size_to_node_size(void *size, struct s_node *node)
{
	size_t *len;

	len = (size_t *)size;
	if (*len < node->m.size)
		return (-1);
	if (*len > node->m.size)
		return (1);
	return (0);
}
