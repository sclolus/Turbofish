/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lst_invert_rec.c                                   :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/04/10 17:33:33 by bmickael          #+#    #+#             */
/*   Updated: 2017/04/26 01:14:20 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "chained_tools.h"

static struct s_list	*invert(struct s_list **alst, struct s_list *ptr)
{
	if (!ptr->next)
		*alst = ptr;
	else
		invert(alst, ptr->next)->next = ptr;
	return (ptr);
}

struct s_list			*lst_invert_rec(struct s_list **alst)
{
	invert(alst, *alst)->next = NULL;
	return (*alst);
}
