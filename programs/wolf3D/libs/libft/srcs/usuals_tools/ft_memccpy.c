/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   ft_memccpy.c                                       :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/04/10 14:32:24 by bmickael          #+#    #+#             */
/*   Updated: 2017/04/10 14:33:43 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "libft.h"

void	*ft_memccpy(void *restrict dst, const void *restrict src, int c,
																	size_t n)
{
	size_t			i;
	unsigned char	*src1;
	unsigned char	*dst1;

	src1 = (unsigned char *)src;
	dst1 = (unsigned char *)dst;
	c = (unsigned char)c;
	i = 0;
	while (i < n)
	{
		dst1[i] = src1[i];
		if (src1[i] == c)
			return ((void *)(dst1 + i + 1));
		i++;
	}
	return (NULL);
}
