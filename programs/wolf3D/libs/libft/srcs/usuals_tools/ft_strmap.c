/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   ft_strmap.c                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/04/10 16:41:59 by bmickael          #+#    #+#             */
/*   Updated: 2017/04/10 16:45:06 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include <stdlib.h>
#include "libft.h"

char	*ft_strmap(char const *s, char (*f)(char))
{
	int		length;
	char	*output;
	char	*t;

	length = ft_strlen(s);
	if ((output = (char *)malloc((length + 1) * sizeof(char))))
	{
		t = output;
		while (length--)
			*t++ = f(*s++);
		*t = '\0';
		return (output);
	}
	return (NULL);
}
