/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   ft_strnstr.c                                       :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/04/10 16:05:46 by bmickael          #+#    #+#             */
/*   Updated: 2017/04/26 01:39:09 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "libft.h"

char	*ft_strnstr(const char *big, const char *little, size_t len)
{
	size_t i;

	i = ft_strlen(little);
	if (i == 0)
		return ((char *)big);
	while (*big && len >= i)
	{
		if (ft_strncmp(big, little, i) == 0)
			return ((char *)big);
		len--;
		big++;
	}
	return (NULL);
}
