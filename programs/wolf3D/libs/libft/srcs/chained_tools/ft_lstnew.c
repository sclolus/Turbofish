/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   ft_lstnew.c                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/04/10 17:20:59 by bmickael          #+#    #+#             */
/*   Updated: 2017/04/10 17:22:00 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include <stdlib.h>
#include "libft.h"

t_list		*ft_lstnew(void const *content, size_t content_size)
{
	t_list *output;

	if ((output = (t_list *)malloc(sizeof(t_list))))
	{
		output->next = NULL;
		if (!content)
		{
			output->content = NULL;
			output->content_size = 0;
			return (output);
		}
		else if ((output->content = (char *)
			malloc(content_size * sizeof(char))))
		{
			ft_memcpy(output->content, content, content_size);
			output->content_size = content_size;
			return (output);
		}
		free(output);
	}
	return (NULL);
}
