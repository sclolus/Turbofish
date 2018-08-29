/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   btree.h                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/03/24 01:42:50 by bmickael          #+#    #+#             */
/*   Updated: 2017/03/24 02:37:21 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#ifndef BTREE_H
# define BTREE_H

/*
** This PACK provide a suitable interface to manipulate binary tree.
** Including black and white color self balanced.
*/

# include <stdlib.h>

struct s_node;

enum	e_check_result {
	FAILED = 0,
	OK
};

struct			s_rnb_tree_checker_result {
	enum e_check_result root_is_black;
	enum e_check_result homogenetic_black;
	enum e_check_result filiation;
	enum e_check_result rnb_interlacement;
	int					nb_levels;
	int					nb_nodes;
};

enum			e_node_register {
	NODE_NEW,
	NODE_ALREADY_PRESENT,
};

/*
** --- CONSTRUCTOR / DESTROYER / ATOMICS ---
*/

/*
** Return a initialized tree
** Return:
** (NULL)
*/

struct s_node	*btree_new(void);

/*
** Destroy an entire tree
** Return:
** SUCCESS: 0
** FAIL: -1 Null deallocator specified
*/

int				btree_delete(struct s_node *root, void (*deallocator)(void *));

/*
** Simple getter of size of a generic node.
** Return:
** sizeof(struct s_node)
*/

size_t			btree_get_node_size(void);

/*
** Create a orphaned node associated with his own content.
** Return:
** SUCCESS: Allocated new orphaned node
** FAIL: (NULL) ENOMEM or Null allocator specified
*/

struct s_node	*btree_create_node(void *content, void *(*allocator)(size_t));

/*
** Simple getter of content.
** Return:
** node->content
*/

void			*btree_get_node_content(struct s_node *node);

/*
** Check if similar value are generated.
** Return:
** SUCCESS: pointer structure s_node if insertion success.
** FAIL: (NULL)
*/

struct s_node	*btree_smash_checker(
		struct s_node **root,
		void *content,
		int (*cmpf)(void *, void *),
		void *(*allocator)(size_t));

/*
** --- These below methods are very dangerous if node is already in a tree,
** you can destroy every-things. ---
*/

/*
** Attach a new content into a node.
** Deallocate old content if necessary.
** Return:
** SUCCESS: 0
** FAIL: -EINVAL Null node sended
*/

int				btree_attach_content(
		struct s_node *node,
		void *content,
		void (*deallocator)(void *));

/*
** Simply destroy a node.
** Return:
** SUCCESS: 0
** FAIL: -EINVAL Null node or Null deallocator sended
*/

int				btree_destoy_node(
		struct s_node *node,
		void (*deallocator)(void *));

/*
** Delete content of a node.
** Return:
** SUCCESS: 0
** FAIL: -EINVAL Null node or Null deallocator or Null node content sended
*/

int				btree_delete_node_content(
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

struct s_node	*btree_insert_node_by_content(
		struct s_node **root,
		void *content,
		int (*cmpf)(void *, void *),
		void *(*allocator)(size_t));

struct s_node	*btree_insert_node(
		struct s_node **root,
		struct s_node *new,
		int (*cmpf)(void *, void *));

struct s_node	*btree_conditional_insert(
		struct s_node **root,
		void *content,
		int (*cmpf)(void *, void *),
		void *(*allocator)(size_t));

/*
** Theses methods provide a node deletion service without Red and Black feature.
** Return:
** SUCESS: 0
** FAIL: -EINVAL
*/

int				btree_delete_node_by_content(
		struct s_node **root,
		void *content,
		int (*cmpf)(void *, void *),
		void (*deallocator)(void *));

int				btree_delete_node(struct s_node **root, struct s_node *node,
		void (*deallocator)(void *));

/*
** --- Providing a fast way to add or remove red and black binary tree node.
** Be careful, mixing classical and black and white nodes lead to many bugs. ---
*/

/*
** Theses methods provide a node insertion service with Red and Black feature.
** Return:
** SUCCESS: pointer structure s_node if insertion success -> new node
** FAIL: (NULL)
*/

struct s_node	*btree_insert_rnb_node_by_content(
		struct s_node **root,
		void *content,
		int (*cmpf)(void *, void *),
		void *(*allocator)(size_t));

struct s_node	*btree_insert_rnb_node(
		struct s_node **root,
		struct s_node *new,
		int (*cmpf)(void *, void *));

/*
** Theses methods provide a node deletion service without Red and Black feature.
** Return:
** SUCESS: 0
** FAIL: -EINVAL or -1 Mode to trash was not founded !
*/

int				btree_delete_rnb_node_by_content(
		struct s_node **root,
		void *content,
		int (*cmpf)(void *, void *),
		void (*deallocator)(void *));

int				btree_delete_rnb_node(
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

int				btree_check_rnb_property(struct s_node *root,
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

struct s_node	*btree_get_node_by_content(
		struct s_node *root,
		void *data_ref,
		int (*cmpf)(void *, void *));

struct s_node	*btree_get_last_valid_node(
		struct s_node *root,
		void *data_ref,
		int (*cmpf)(void *, void *));

void			*btree_get_last_valid_content(
		struct s_node *root,
		void *data_ref,
		int (*cmpf)(void *, void *));

struct s_node	*btree_get_next_neighbours_node(struct s_node *node);

struct s_node	*btree_get_prev_neighbours_node(struct s_node *node);

void			*btree_get_next_neighbours_content(struct s_node *node);

void			*btree_get_prev_neighbours_content(struct s_node *node);

struct s_node	*btree_get_highest_node(struct s_node *node);
struct s_node	*btree_get_lowest_node(struct s_node *node);
void			*btree_get_highest_node_content(struct s_node *node);
void			*btree_get_lowest_node_content(struct s_node *node);

int				btree_is_last_node(struct s_node *node);
/*
** Return a specified content or Fn equ similary.
** Return:
** SUCCESS: founded content
** FAIL: (NULL) If root or cmp fn are Null OR Not was not founded !
*/

void			*btree_search_content(struct s_node *root, void *data_ref,
		int (*cmpf)(void *, void *));

/*
** --- Providing three ways to run away the binary tree.
** Infix method may be more interesting. It's sort elements.
** Return:
** SUCCESS: 0
** FAIL: -EINVAL Null function pointer sended --
*/

int				btree_apply_infix(struct s_node *root,
		void (*applyf)(void *));
int				btree_apply_prefix(struct s_node *root,
		void (*applyf)(void *));
int				btree_apply_suffix(struct s_node *root,
		void (*applyf)(void *));

/*
** --- Just check if the binary tree is not broken.
** May check if values in a tree are sorted.
** Return:
** SUCCESS: Custom
** FAIL: -EINVAL Null root sended ---
*/

int				btree_check_binary_tree(
		struct s_node *root, int (*applyf)(void *));

/*
** --- Some extra functions for binary tree ---
*/

int				btree_memory_move(void *dest, struct s_node *src_node);

int				btree_apply_by_level(struct s_node *root,
		void (*applyf)(void *content, int current_level, int first_elem));

int				btree_level_count(struct s_node *root);

#endif
