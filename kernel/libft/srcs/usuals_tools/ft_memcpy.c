/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   ft_memcpy.c                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/04/10 14:27:36 by bmickael          #+#    #+#             */
/*   Updated: 2017/04/26 01:30:19 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "libft.h"

void	*ft_memcpy(void *restrict dst, const void *restrict src, size_t n)
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
