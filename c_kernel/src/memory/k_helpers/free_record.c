
#include "main_headers.h"

static struct s_node	*get_parent(
			size_t size,
			enum e_page_type type)
{
	struct s_node_params	use_ctx;
	struct s_node		*parent;

	use_ctx.allocator = &node_custom_allocator;
	use_ctx.comp = &cmp_size_to_node_size;
	use_ctx.associator = (type == TINY) ?
			&assign_parent_free_tiny :
			&assign_parent_free_medium;
	parent = alloc_btree_try_to_insert_rnb_node((type == TINY) ?
			&ctx.global_tiny_space_tree :
			&ctx.global_medium_space_tree,
			&size, &use_ctx);
	return (parent);
}

/*
** First, check_index_destroy verify if the free slot is not for the total page
** size: If it is the case, the indexed page may be totally destroyed !
** After, allocate a new free parent record if necessary and push his son !
*/

int		insert_free_record(
		void *addr,
		size_t size,
		enum e_page_type type,
		struct s_node **parent_ref)
{
	struct s_node	*parent;
	struct s_node	*record;

	if (check_index_destroy(addr, size, type))
		return (0);
	if ((parent = get_parent(size, type)) == NULL)
		return (-1);
	if (parent_ref)
		*parent_ref = parent;
	if ((record = alloc_btree_create_node(&node_custom_allocator))
			== NULL) {
		if (parent->ptr_a == NULL)
			alloc_btree_delete_rnb_node_by_content((type == TINY) ?
					&ctx.global_tiny_space_tree :
					&ctx.global_medium_space_tree,
					&size,
					&cmp_size_to_node_size,
					&node_custom_deallocator);
		return (-1);
	}
	record->m.size = size;
	record->ptr_a = addr;
	record->mask.s.node_type = (type == TINY) ?
			RECORD_FREE_TINY : RECORD_FREE_MEDIUM;
	record = alloc_btree_insert_rnb_node(
			((struct s_node **)&parent->ptr_a),
			record,
			&cmp_node_addr_to_node_addr);
	return (0);
}

struct s_node	*get_free_record(
		void *addr,
		size_t size,
		struct s_node **parent,
		enum e_page_type type)
{
	struct s_node	*out;

	*parent = alloc_btree_get_node_by_content((type == TINY) ?
			ctx.global_tiny_space_tree :
			ctx.global_medium_space_tree,
			&size, &cmp_size_to_node_size);
	if (*parent == NULL)
		return (NULL);
	out = alloc_btree_get_node_by_content(
			((struct s_node *)(*parent)->ptr_a),
			addr,
			&cmp_addr_to_node_addr);
	if (out == NULL)
		return (NULL);
	return (out);
}

int		delete_free_record(
		struct s_node *record,
		struct s_node *parent,
		enum e_page_type type)
{
	size_t	size;
	int	ret;

	size = record->m.size;
	ret = alloc_btree_delete_rnb_node(((struct s_node **)&parent->ptr_a),
			record, &node_custom_deallocator);
	if (ret == 0) {
		alloc_btree_delete_rnb_node_by_content(
				(type == TINY) ?
						&ctx.global_tiny_space_tree :
						&ctx.global_medium_space_tree,
				&size,
				&cmp_size_to_node_size,
				&node_custom_deallocator);
	}
	return (0);
}

struct s_node	*get_best_free_record_tree(
		size_t size,
		enum e_page_type type)
{
	struct s_node	*parent;
	struct s_node	*index;
	void		*addr;
	size_t		range;

	if ((parent = alloc_btree_get_last_valid_node((type == TINY) ?
			ctx.global_tiny_space_tree :
			ctx.global_medium_space_tree, &size,
			&cmp_size_to_node_size)) != NULL)
		return (parent);
	range = type == TINY ? TINY_RANGE : MEDIUM_RANGE;
	if ((addr = get_new_pages(range)) == NULL)
		return (NULL);
	index = create_index(addr, range);
	if (index == NULL) {
		destroy_pages(addr, range);
		return (NULL);
	}
	if (insert_free_record(addr, range, type, &parent) < 0) {
		destroy_index(index);
		destroy_pages(addr, range);
		return (NULL);
	}
	return (parent);
}

void		assign_parent_free_tiny(
		void *content,
		struct s_node *node)
{
	size_t *size;

	size = (size_t *)content;
	node->m.size = *size;
	node->ptr_a = NULL;
	node->mask.s.node_type = PARENT_RECORD_FREE_TINY;
}

void		assign_parent_free_medium(
		void *content,
		struct s_node *node)
{
	size_t *size;

	size = (size_t *)content;
	node->m.size = *size;
	node->ptr_a = NULL;
	node->mask.s.node_type = PARENT_RECORD_FREE_MEDIUM;
}

/*
** The aim of this function is to avoid having two full free pages:
** Regarding to TINY_RANGE or MEDIUM_RANGE, this function destroy
** a index page if his record is totally free and a total free record
** exists already.
** Return 1 if the suppress operation has been done. In this case,
** 			no new free chunk will be created.
** Return 0 if there are no free node like that, it will be
** 			created in caller.
*/

int		check_index_destroy(
		void *addr,
		size_t size,
		enum e_page_type type)
{
	struct s_node *parent;
	struct s_node *index;

	parent = NULL;
	if (type == TINY && size == TINY_RANGE)
		parent = alloc_btree_get_node_by_content(
				ctx.global_tiny_space_tree,
				&size, &cmp_size_to_node_size);
	if (type == MEDIUM && size == MEDIUM_RANGE)
		parent = alloc_btree_get_node_by_content(
				ctx.global_medium_space_tree,
				&size, &cmp_size_to_node_size);
	if (parent) {
		index = alloc_btree_get_node_by_content(
				ctx.index_pages_tree,
				addr,
				&cmp_m_addr_to_node_m_addr);
		destroy_index(index);
		destroy_pages(addr, size);
		return (1);
	}
	return (0);
}
