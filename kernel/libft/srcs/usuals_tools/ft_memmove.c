/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   ft_memmove.c                                       :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/04/10 14:36:08 by bmickael          #+#    #+#             */
/*   Updated: 2017/04/16 15:43:25 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "libft.h"

void	*ft_memmove(void *dst, const void *src, size_t len)
{
	char *s1;
	char *s2;

	if (src == dst)
		return ((void *)src);
	if (src < dst)
	{
		s1 = (char *)src + len - 1;
		s2 = (char *)dst + len - 1;
		while (len--)
			*s2-- = *s1--;
	}
	else
	{
		s1 = (char *)src;
		s2 = (char *)dst;
		while (len--)
			*s2++ = *s1++;
	}
	return (dst);
}
