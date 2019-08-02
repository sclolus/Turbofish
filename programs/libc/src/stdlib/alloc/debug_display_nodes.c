
#include "main_headers.h"
#include "stdio.h"

static inline void	show_part(struct s_node_page *e, int i, int fd)
{
	dprintf(fd, "%.3i: ", i);
	dprintf(fd, "{red}%s{eoc}", e->node[i].parent == NULL ?
			"ROOT " : "     ");

	if (e->node[i].mask.s.node_type == INDEX)
		dprintf(fd, "UNIQUE: index_pages_tree, %p", &e->node[i]);
	else if (e->node[i].mask.s.node_type == RECORD_ALLOCATED_LARGE)
		dprintf(fd, "UNIQUE: big_page_record_tree");
	else if (e->node[i].mask.s.node_type == PARENT_RECORD_FREE_TINY)
		dprintf(fd, "UNIQUE: global_tiny_space_tree");
	else if (e->node[i].mask.s.node_type == PARENT_RECORD_FREE_MEDIUM)
		dprintf(fd, "UNIQUE: global_medium_space_tree");
	else if (e->node[i].mask.s.node_type == RECORD_ALLOCATED_TINY)
		dprintf(fd, "        record_allocated_tiny");
	else if (e->node[i].mask.s.node_type == RECORD_ALLOCATED_MEDIUM)
		dprintf(fd, "        record_allocated_medium");
	else if (e->node[i].mask.s.node_type == RECORD_FREE_TINY)
		dprintf(fd, "        record_free_tiny");
	else if (e->node[i].mask.s.node_type == RECORD_FREE_MEDIUM)
		dprintf(fd, "        record_free_medium");
	else
		dprintf(fd, "{red}unknown node{eoc}");
	dprintf(fd, "\n");
}

void			debug_nodes(int fd)
{
	struct s_node_page	*e;
	int			i;

	dprintf(fd, "{red}__ALLOCATED_NODES__:{eoc}\n");
	e = ctx.node_pages_entry;
	while (e) {
		i = 0;
		while (i < e->primary_block.nb_node) {
			show_part(e, i, fd);
			i++;
		}
		e = e->primary_block.next;
		dprintf(fd, "\n");
	}
}
