/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   ft_strlcat.c                                       :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/04/10 15:52:39 by bmickael          #+#    #+#             */
/*   Updated: 2017/04/10 15:53:15 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "libft.h"

size_t	ft_strlcat(char *restrict dst, const char *restrict src, size_t size)
{
	size_t src_len;
	size_t dst_len;

	src_len = 0;
	while (src[src_len])
		src_len++;
	dst_len = 0;
	while (*dst++)
		dst_len++;
	dst -= 1;
	if (dst_len >= size)
		return (size + src_len);
	size -= dst_len + 1;
	while (*src && size--)
		*dst++ = *src++;
	*dst = '\0';
	return (src_len + dst_len);
}
