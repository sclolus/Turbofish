
#include "main_headers.h"

#define STDOUT_FILENO 1

static void		display_alloc(struct s_node *record)
{
	int fd;

	fd = (ctx.tracer_file_descriptor != -1) ?
			ctx.tracer_file_descriptor : STDOUT_FILENO;
	fprintk(fd, "%p --> %p  %lu\n",
			record->ptr_a,
			(uint8_t *)record->ptr_a + record->m.size - 1,
			record->m.size);
}

static void		display_pages_alloc_tiny(struct s_node *index)
{
	int fd;

	if (index->mask.s.range != TINY_RANGE)
		return ;
	fd = (ctx.tracer_file_descriptor != -1) ?
			ctx.tracer_file_descriptor : STDOUT_FILENO;
	fprintk(fd, "{yellow}PAGE: %p{eoc}\n", (void *)index->m.size);
	alloc_btree_apply_infix(
			(struct s_node *)index->ptr_a,
			&display_alloc);
}

static void		display_pages_alloc_medium(struct s_node *index)
{
	int fd;

	if (index->mask.s.range != MEDIUM_RANGE)
		return ;
	fd = (ctx.tracer_file_descriptor != -1) ?
			ctx.tracer_file_descriptor : STDOUT_FILENO;
	fprintk(fd, "{yellow}PAGE: %p{eoc}\n", (void *)index->m.size);
	alloc_btree_apply_infix(
			(struct s_node *)index->ptr_a,
			&display_alloc);
}

static void		display_pages_free(struct s_node *index)
{
	int fd;

	fd = (ctx.tracer_file_descriptor != -1) ?
			ctx.tracer_file_descriptor : STDOUT_FILENO;
	fprintk(fd, "{yellow}chunk size: %lu{eoc}\n",
			(void *)index->m.size);
	alloc_btree_apply_infix(
			(struct s_node *)index->ptr_a,
			&display_alloc);
}

void			show_alloc(bool verbose, int fd)
{
	if (verbose) {
		debug_nodes(fd);
		fprintk(fd, "\n{green}__TINY_FREE_BLOCK__{eoc}\n");
		alloc_btree_apply_infix(
				ctx.global_tiny_space_tree,
				&display_pages_free);
		fprintk(fd, "\n{green}__MEDIUM_FREE_BLOCK__{eoc}\n");
		alloc_btree_apply_infix(
				ctx.global_medium_space_tree,
				&display_pages_free);
		fprintk(fd, "\n");
	}
	fprintk(fd, "{magenta}__TINY_ALLOCATED_BLOCK__{eoc}\n");
	alloc_btree_apply_infix(
			ctx.index_pages_tree,
			&display_pages_alloc_tiny);
	fprintk(fd, "\n{magenta}__MEDIUM_ALLOCATED_BLOCK__{eoc}\n");
	alloc_btree_apply_infix(
			ctx.index_pages_tree,
			&display_pages_alloc_medium);
	fprintk(fd, "\n{magenta}__LARGE_ALLOCATED_BLOCK__{eoc}\n");
	alloc_btree_apply_infix(
			ctx.big_page_record_tree,
			&display_alloc);
	fprintk(fd, "\n{yellow}%lu{eoc} bytes allocated for kernel data,"
			"{yellow} %lu{eoc} bytes"
			" allocated by metadata nodes"
			"total {magenta}%lu{eoc} bytes.\n\n",
			ctx.size_owned_by_data, ctx.size_owned_by_nodes,
			ctx.size_owned_by_data + ctx.size_owned_by_nodes);
}
