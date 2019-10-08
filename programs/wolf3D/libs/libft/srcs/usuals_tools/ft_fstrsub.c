/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   ft_fstrsub.c                                       :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/04/27 22:07:27 by bmickael          #+#    #+#             */
/*   Updated: 2017/04/27 22:07:33 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include <stdlib.h>
#include "libft.h"

char	*ft_fstrsub(char *s, unsigned int start, size_t len)
{
	char *str;
	char *debut;

	if (!(str = (char*)malloc((len + 1) * sizeof(*str))))
		return (NULL);
	debut = str;
	while (s[start] && len > 0)
	{
		*str = s[start];
		start++;
		str++;
		len--;
	}
	*str = '\0';
	return (debut);
}
