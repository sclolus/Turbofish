/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   btree_internal_header.h                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/03/22 14:13:04 by bmickael          #+#    #+#             */
/*   Updated: 2017/03/24 07:10:12 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#ifndef BTREE_INTERNAL_HEADER_H
# define BTREE_INTERNAL_HEADER_H

# include "btree.h"

enum	e_color {
	RED = 0,
	BLACK,
	DOUBLE_BLACK
};

# define EINVAL				1

# define IS_RED(node)		(node && node->color == RED)
# define IS_BLACK(node)		(node == NULL || node->color == BLACK)
# define IS_DB_BLACK(node)	(node && node->color == DOUBLE_BLACK)
# define SET_RED(node)		(node->color = RED)
# define SET_BLACK(node)	(node->color = BLACK)
# define SET_DB_BLACK(node)	(node->color = DOUBLE_BLACK)

struct			s_node {
	struct s_node	*left;
	struct s_node	*right;
	struct s_node	*parent;
	void			*content;
	enum e_color	color;
};

struct s_node	*btree_internal_insert_node_content(
		struct s_node **root,
		void *content,
		int (*cmpf)(void *, void *),
		void *(*allocator)(size_t));

struct s_node	*btree_internal_insert_node(
		struct s_node **root,
		struct s_node *new,
		int (*cmpf)(void *, void *));

struct s_node	*btree_internal_trash_node(
		struct s_node *node,
		struct s_node **root,
		struct s_node **sibling);

void			apply_insert_strategy(struct s_node *new);

void			apply_delete_strategy(
		struct s_node *trash,
		struct s_node **root,
		struct s_node *sibling);

/*
** Internal use.
*/

void			btree_rotate_right(struct s_node *n);
void			btree_rotate_left(struct s_node *n);

struct s_node	*btree_get_parent(struct s_node *n);
struct s_node	*btree_get_grandparent(struct s_node *n);
struct s_node	*btree_get_sibling(struct s_node *n);
struct s_node	*btree_get_uncle(struct s_node *n);

void			minor_rotations_case(
		struct s_node *sibling,
		struct s_node **root);

void			major_rotations_case(
		struct s_node *sibling,
		struct s_node **root);

#endif
