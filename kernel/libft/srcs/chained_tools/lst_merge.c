/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lst_merge.c                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/04/10 17:30:54 by bmickael          #+#    #+#             */
/*   Updated: 2017/04/10 17:31:04 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "chained_tools.h"

void	lst_merge(struct s_list **alst, struct s_list *lst)
{
	struct s_list *ptr;

	if (!(*alst))
	{
		*alst = lst;
		return ;
	}
	ptr = *alst;
	while (ptr->next)
		ptr = ptr->next;
	ptr->next = lst;
}
