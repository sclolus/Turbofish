/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lst_new.c                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/04/10 17:20:59 by bmickael          #+#    #+#             */
/*   Updated: 2017/04/10 17:22:00 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "chained_tools.h"

static void		*internal_memcpy(
		void *restrict dst,
		const void *restrict src,
		size_t n)
{
	char *src1;
	char *dst1;

	if (src == dst)
		return ((void *)src);
	src1 = (char *)src;
	dst1 = (char *)dst;
	while (n--)
		*dst1++ = *src1++;
	return (dst);
}

struct s_list	*lst_new(
		void const *content,
		size_t content_size,
		void *(*allocator)(size_t),
		void (*deallocator)(void *))
{
	struct s_list *output;

	if ((output = (struct s_list *)allocator(sizeof(struct s_list))))
	{
		output->next = NULL;
		if (!content)
		{
			output->content = NULL;
			output->content_size = 0;
			return (output);
		}
		else if ((output->content = (char *)
			allocator(content_size * sizeof(char))))
		{
			internal_memcpy(output->content, content, content_size);
			output->content_size = content_size;
			return (output);
		}
		deallocator(output);
	}
	return (NULL);
}
