
#include "main_headers.h"

int	cmp_addr_to_node_m_addr_range(void *content, struct s_node *node)
{
	if (content < node->m.ptr_b)
		return (-1);
	if ((uint8_t *)content >= (uint8_t *)node->m.ptr_b + node->mask.s.range)
		return (1);
	return (0);
}

int	cmp_node_m_addr_to_node_m_addr(
	struct s_node *node_a,
	struct s_node *node_b)
{
	if (node_a->m.ptr_b < node_b->m.ptr_b)
		return (-1);
	if (node_a->m.ptr_b > node_b->m.ptr_b)
		return (1);
	return (0);
}

int	cmp_m_addr_to_node_m_addr(void *addr, struct s_node *node_b)
{
	if (addr < node_b->m.ptr_b)
		return (-1);
	if (addr > node_b->m.ptr_b)
		return (1);
	return (0);
}
