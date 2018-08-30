/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lst_map.c                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/04/10 17:26:56 by bmickael          #+#    #+#             */
/*   Updated: 2017/04/12 02:36:10 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "chained_tools.h"

static void		del_elem(void *elem, size_t size, void (*deallocator)(void *))
{
	(void)size;
	deallocator(elem);
	elem = NULL;
}

struct s_list	*lst_map(
	struct s_list *lst,
	struct s_list *(*f)(struct s_list *elem),
	void (*deallocator)(void *))
{
	struct s_list *current;
	struct s_list *elem;
	struct s_list *new_list;

	new_list = NULL;
	current = lst;
	while (current)
	{
		if (!(elem = f(current)))
		{
			lst_del(&new_list, &del_elem, deallocator);
			return (NULL);
		}
		lst_add(&new_list, elem);
		current = current->next;
	}
	return (lst_invert_it(&new_list));
}
