/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lst_iter.c                                         :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/04/10 17:25:43 by bmickael          #+#    #+#             */
/*   Updated: 2017/04/10 17:25:47 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "chained_tools.h"

void	lst_iter(struct s_list *lst, void (*f)(struct s_list *elem))
{
	struct s_list *current;

	current = lst;
	while (current)
	{
		f(current);
		current = current->next;
	}
}
