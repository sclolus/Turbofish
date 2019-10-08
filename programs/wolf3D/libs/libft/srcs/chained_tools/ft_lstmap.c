/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   ft_lstmap.c                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/04/10 17:26:56 by bmickael          #+#    #+#             */
/*   Updated: 2017/04/12 02:36:10 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include <stdlib.h>
#include "libft.h"

static void		del_elem(void *elem, size_t size)
{
	(void)size;
	free(elem);
	elem = NULL;
}

t_list			*ft_lstmap(t_list *lst, t_list *(*f)(t_list *elem))
{
	t_list *current;
	t_list *elem;
	t_list *new_list;

	new_list = NULL;
	current = lst;
	while (current)
	{
		if (!(elem = f(current)))
		{
			ft_lstdel(&new_list, &del_elem);
			return (NULL);
		}
		ft_lstadd(&new_list, elem);
		current = current->next;
	}
	return (ft_lst_invert_it(&new_list));
}
