
#ifndef BTREE_H
# define BTREE_H

/*
** This PACK provide a suitable interface to manipulate binary tree.
** Including black and white color self balanced.
*/

# include "i386_type.h"

struct s_node;

enum				e_check_result {
	FAILED = 0,
	OK
};

struct				s_rnb_tree_checker_result {
	enum e_check_result	root_is_black;
	enum e_check_result	homogenetic_black;
	enum e_check_result	filiation;
	enum e_check_result	rnb_interlacement;
	int			nb_levels;
	int			nb_nodes;
};

enum				e_color {
	RED = 0,
	BLACK,
	DOUBLE_BLACK
};

enum				e_page_type {
	TINY = 0,
	MEDIUM,
	LARGE
};

enum				e_node_type {
	RECORD_ALLOCATED_TINY = 0,
	RECORD_ALLOCATED_MEDIUM,
	RECORD_ALLOCATED_LARGE,
	INDEX,
	PARENT_RECORD_FREE_TINY,
	PARENT_RECORD_FREE_MEDIUM,
	RECORD_FREE_TINY,
	RECORD_FREE_MEDIUM,
};

# define IS_RED(node)		(node && (node->mask.s.color == RED))
# define IS_BLACK(node)		(node == NULL || (node->mask.s.color == BLACK))
# define IS_DB_BLACK(node)	(node && (node->mask.s.color == DOUBLE_BLACK))
# define SET_RED(node)		(node->mask.s.color = RED)
# define SET_BLACK(node)	(node->mask.s.color = BLACK)
# define SET_DB_BLACK(node)	(node->mask.s.color = DOUBLE_BLACK)

struct				s_mask {
	uint8_t			color;
	uint8_t			node_type;
	uint8_t			unused_a;
	uint8_t			unused_b;
	uint32_t		range;
};

union				u_mask {
	uint32_t		raw;
	struct s_mask		s;
};

union				u_ptr {
	size_t			size;
	void			*ptr_b;
};

struct				s_node {
	struct s_node		*left;
	struct s_node		*right;
	struct s_node		*parent;
	void			*ptr_a;
	union u_ptr		m;
	union u_mask		mask;
} __attribute__((aligned(16)));

enum				e_node_register {
	ERROR,
	NODE_ALREADY_PRESENT,
	NODE_ALLOCATED,
};

struct				s_node_params {
	void			*(*allocator)(size_t);
	void			(*associator)(void *, struct s_node *);
	int			(*comp)(void *, struct s_node *);
	enum e_node_register	reg;
};

/*
** XXX Define this function is always necessary.
*/

void				alloc_btree_swap_data(
				struct s_node *node_b,
				struct s_node *node_a);

/*
** --- CONSTRUCTOR / DESTROYER / ATOMICS ---
*/

/*
** Return a initialized tree
** Return:
** (NULL)
*/

struct s_node			*alloc_btree_new(void);

/*
** Destroy an entire tree
** Return:
** SUCCESS: 0
** FAIL: -1 Null deallocator specified
*/

int				alloc_btree_delete(
				struct s_node *root,
				void (*deallocator)(void *));

/*
** Simple getter of size of a generic node.
** Return:
** sizeof(struct s_node)
*/

size_t				alloc_btree_get_node_size(void);

/*
** Create a orphaned node.
** Return:
** SUCCESS: Allocated new orphaned node
** FAIL: (NULL) ENOMEM or Null allocator specified
*/

struct s_node			*alloc_btree_create_node(
				void *(*allocator)(size_t));

/*
** Check if similar value are generated.
** Return:
** SUCCESS: pointer structure s_node if insertion success.
** FAIL: (NULL)
*/

struct s_node			*alloc_btree_smash_checker(
				struct s_node **root,
				void *content,
				int (*cmpf)(void *, struct s_node *),
				void *(*allocator)(size_t));

/*
** --- These below methods are very dangerous if node is already in a tree,
** you can destroy every-things. ---
*/

/*
** Simply destroy a node.
** Return:
** SUCCESS: 0
** FAIL: -EINVAL Null node or Null deallocator sended
*/

int				alloc_btree_destroy_node(
				struct s_node *node,
				void (*deallocator)(void *));

/*
** --- Providing a fast way to add or remove classical binary tree node.
** Be careful, mixing classical and black and white nodes lead to many bugs. ---
*/

/*
** Theses methods provide a node insert service without Red and Black feature.
** Return:
** SUCCESS: pointer structure s_node if insertion success -> new node
** FAIL: (NULL)
*/

struct s_node			*alloc_btree_insert_node(
				struct s_node **root,
				struct s_node *new,
				int (*cmpf)(struct s_node *, struct s_node *));

/*
** Theses methods provide a node deletion service without Red and Black feature.
** Return:
** SUCESS: 0
** FAIL: -EINVAL
*/

int				alloc_btree_delete_node_by_content(
				struct s_node **root,
				void *content,
				int (*cmpf)(void *, struct s_node *),
				void (*deallocator)(void *));

int				alloc_btree_delete_node(
				struct s_node **root,
				struct s_node *node,
				void (*deallocator)(void *));

/*
** --- Providing a fast way to add or remove red and black binary tree node.
** Be careful, mixing classical and black and white nodes lead to many bugs.
*/

/*
** Theses methods provide a node insertion service with Red and Black feature.
** Return:
** SUCCESS: pointer structure s_node if insertion success -> new node
** FAIL: (NULL)
*/

struct s_node			*alloc_btree_insert_rnb_node(
				struct s_node **root,
				struct s_node *new,
				int (*cmpf)(struct s_node *, struct s_node *));

struct s_node			*alloc_btree_try_to_insert_rnb_node(
				struct s_node **root,
				void *content,
				struct s_node_params *use_ctx);

/*
** Theses methods provide a node deletion service without Red and Black feature.
** Return:
** SUCESS: 0
** FAIL: -EINVAL or -1 Mode to trash was not founded !
*/

int				alloc_btree_delete_rnb_node_by_content(
				struct s_node **root,
				void *content,
				int (*cmpf)(void *, struct s_node *),
				void (*deallocator)(void *));

int				alloc_btree_delete_rnb_node(
				struct s_node **root,
				struct s_node *node,
				void (*deallocator)(void *));

/*
** --- Check if a Red and Black tree respect standard rules.
** The reference structure s_rnb_tree_checker_result got results:
** enum e_check_result root_is_black;     ->
** enum e_check_result homogenetic_black; -> Blacks ways got similar length.
** enum e_check_result filiation;         -> Sons know their parent.
** enum e_check_result rnb_interlacement; -> No dual Red neighbors.
** int                 nb_levels;         -> Deep of the tree.
** int                 nb_nodes;          -> Total of nodes.
** For enum e_check_result, states are OK or FAILED.
** Return:
** SUCESS: 0
** FAIL: -EINVAL Null structure pointer sended ---
*/

int				alloc_btree_check_rnb_property(
				struct s_node *root,
				struct s_rnb_tree_checker_result *result);

/*
** -- Main getters. ---
*/

/*
** Return the node witch the specified content or Fn equ similary.
** Return:
** SUCCESS: pointer structure s_node
** FAIL: (NULL) If root or cmp fn are Null OR Not was not founded !
*/

struct s_node			*alloc_btree_get_node_by_content(
				struct s_node *root,
				void *data_ref,
				int (*cmpf)(void *, struct s_node *));

struct s_node			*alloc_btree_get_last_valid_node(
				struct s_node *root,
				void *data_ref,
				int (*cmpf)(void *, struct s_node *));

struct s_node			*alloc_btree_get_next_neighbours_node(
				struct s_node *node);

struct s_node			*alloc_btree_get_prev_neighbours_node(
				struct s_node *node);

struct s_node			*alloc_btree_get_highest_node(
				struct s_node *node);
struct s_node			*alloc_btree_get_lowest_node(
				struct s_node *node);

int				alloc_btree_is_last_node(struct s_node *node);

/*
** --- Providing three ways to run away the binary tree.
** Infix method may be more interesting. It's sort elements.
** Return:
** SUCCESS: 0
** FAIL: -EINVAL Null function pointer sended --
*/

int				alloc_btree_apply_infix(
				struct s_node *root,
				void (*applyf)(struct s_node *node));
int				alloc_btree_apply_infix(
				struct s_node *root,
				void (*applyf)(struct s_node *node));
int				alloc_btree_apply_suffix(
				struct s_node *root,
				void (*applyf)(struct s_node *node));

/*
** --- Just check if the binary tree is not broken.
** May check if values in a tree are sorted.
** Return:
** SUCCESS: Custom
** FAIL: -EINVAL Null root sended ---
*/

int				alloc_btree_check_binary_tree(
				struct s_node *root,
				int (*applyf)(struct s_node *));

/*
** --- Some extra functions for binary tree ---
*/

int				alloc_btree_memory_move(
				void *dest,
				struct s_node *src_node);

int				alloc_btree_apply_by_level(
				struct s_node *root,
				void (*applyf)(
						struct s_node *node,
						int current_level,
						int first_elem));

int				alloc_btree_level_count(struct s_node *root);

#endif
