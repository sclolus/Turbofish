/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   st_invert_it.c                                     :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/04/10 17:30:10 by bmickael          #+#    #+#             */
/*   Updated: 2017/04/10 17:30:13 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "chained_tools.h"

struct s_list		*lst_invert_it(struct s_list **alst)
{
	struct s_list *p;
	struct s_list *c;
	struct s_list *n;

	p = NULL;
	c = *alst;
	while (c)
	{
		n = c->next;
		c->next = p;
		p = c;
		c = n;
	}
	*alst = p;
	return (*alst);
}
