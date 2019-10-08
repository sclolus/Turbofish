/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   ft_lst_pre_alloc.c                                 :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/04/10 17:31:29 by bmickael          #+#    #+#             */
/*   Updated: 2017/04/10 17:31:45 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include <stdlib.h>
#include "libft.h"

t_list		*ft_lst_pre_alloc(t_list **alst, size_t len)
{
	t_list *m;

	if (!(m = (t_list *)malloc(sizeof(t_list))))
		return (NULL);
	if (!(m->content = (char *)malloc(len)))
	{
		free(m);
		return (NULL);
	}
	m->content_size = len;
	if (!(*alst))
	{
		*alst = m;
		m->next = NULL;
		return (*alst);
	}
	m->next = *alst;
	*alst = m;
	return (*alst);
}
