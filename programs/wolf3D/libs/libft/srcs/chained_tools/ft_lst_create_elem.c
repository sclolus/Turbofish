/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   ft_lst_create_elem.c                               :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/04/10 17:29:12 by bmickael          #+#    #+#             */
/*   Updated: 2017/04/10 17:29:23 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include <stdlib.h>
#include "libft.h"

t_list		*ft_lst_create_elem(void *data, size_t len)
{
	t_list *elmt;

	if (!(elmt = (t_list *)malloc(sizeof(t_list))))
		return (NULL);
	elmt->content = data;
	elmt->content_size = len;
	return (elmt);
}
