/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   chained_tools.h                                    :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/03/24 01:42:50 by bmickael          #+#    #+#             */
/*   Updated: 2017/03/24 02:37:21 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#ifndef CHAINED_TOOLS_H
# define CHAINED_TOOLS_H

# include <stdlib.h>

struct s_list;

struct				s_list
{
	void			*content;
	size_t			content_size;
	struct s_list	*next;
};

struct s_list		*lst_new(
					void const *content,
					size_t content_size,
					void *(*allocator)(size_t),
					void (*deallocator)(void *));

void				lst_del_one(
					struct s_list **alst,
					void (*del)(void *, size_t),
					void (*deallocator)(void *));

void				lst_del(
					struct s_list **alst,
					void (*del)(void *, size_t, void (*)(void *)),
					void (*deallocator)(void *));

void				lst_add(struct s_list **alst, struct s_list *new);

void				lst_iter(
					struct s_list *lst,
					void (*f)(struct s_list *elem));

struct s_list		*lst_map(
					struct s_list *lst,
					struct s_list *(*f)(struct s_list *elem),
					void (*deallocator)(void *));

struct s_list		*lst_invert_rec(struct s_list **alst);

struct s_list		*lst_push_front(
					struct s_list **alst,
					void *data,
					size_t len,
					void *(*allocator)(size_t));

struct s_list		*lst_push_back(
					struct s_list **alst,
					void *data,
					size_t len,
					void *(*allocator)(size_t));

struct s_list		*lst_pre_alloc(
					struct s_list **alst,
					size_t len,
					void *(*allocator)(size_t),
					void (*deallocator)(void *));

struct s_list		*lst_create_elem(
					void *data,
					size_t len,
					void *(*allocator)(size_t));

struct s_list		*lst_invert_it(struct s_list **alst);

void				lst_merge(struct s_list **alst, struct s_list *lst);

#endif
