/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lst_del_one.c                                      :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/04/10 17:22:57 by bmickael          #+#    #+#             */
/*   Updated: 2017/04/10 17:23:12 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "chained_tools.h"

void	lst_del_one(struct s_list **alst, void (*del)(void *, size_t),
		void (*deallocator)(void *))
{
	del((*alst)->content, (*alst)->content_size);
	deallocator(*alst);
	*alst = NULL;
}
