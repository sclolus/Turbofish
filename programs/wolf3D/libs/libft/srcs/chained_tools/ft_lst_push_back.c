/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   ft_lst_push_back.c                                 :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/04/10 17:32:12 by bmickael          #+#    #+#             */
/*   Updated: 2017/04/10 17:32:19 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "libft.h"

t_list		*ft_lst_push_back(t_list **alst, void *data, size_t len)
{
	t_list *m;
	t_list *ptr;

	if (!(m = ft_lst_create_elem(data, len)))
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
