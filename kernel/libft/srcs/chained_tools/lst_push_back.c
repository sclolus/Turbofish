/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lst_push_back.c                                    :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/04/10 17:32:12 by bmickael          #+#    #+#             */
/*   Updated: 2017/04/10 17:32:19 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "chained_tools.h"

struct s_list	*lst_push_back(
		struct s_list **alst,
		void *data,
		size_t len,
		void *(*allocator)(size_t))
{
	struct s_list *m;
	struct s_list *ptr;

	if (!(m = lst_create_elem(data, len, allocator)))
		return (NULL);
	if (!(*alst))
	{
		*alst = m;
		(*alst)->next = NULL;
		return (*alst);
	}
	ptr = *alst;
	while (ptr->next)
		ptr = ptr->next;
	ptr->next = m;
	m->next = NULL;
	return (*alst);
}
