/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   ft_strnew.c                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/04/10 16:36:29 by bmickael          #+#    #+#             */
/*   Updated: 2017/04/10 20:23:18 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include <stdlib.h>
#include "libft.h"

char	*ft_strnew(size_t size)
{
	char *output;
	char *origin;

	size += 1;
	if ((output = (char *)malloc(size * sizeof(char))))
	{
		origin = output;
		while (size--)
			*output++ = '\0';
		return (origin);
	}
	return (NULL);
}
