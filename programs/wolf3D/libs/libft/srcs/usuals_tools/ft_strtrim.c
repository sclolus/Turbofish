/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   ft_strtrim.c                                       :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/04/10 16:52:39 by bmickael          #+#    #+#             */
/*   Updated: 2017/04/16 16:00:36 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include <stdlib.h>
#include "libft.h"

static char		*s_getbegin(char *s)
{
	char *origin;

	origin = s;
	while (*s)
	{
		if (*s != ' ' && *s != '\t' && *s != '\n')
			return (s);
		s++;
	}
	return (origin);
}

static char		*s_getend(char *s)
{
	size_t size;

	if (!(size = ft_strlen(s)))
		return (s);
	s = s + size;
	while (size--)
	{
		s--;
		if (*s != ' ' && *s != '\t' && *s != '\n')
			return (s + 1);
	}
	return (s);
}

char			*ft_strtrim(const char *s)
{
	char *ptrbeg;
	char *ptrend;
	char *str;
	char *begin;

	ptrbeg = s_getbegin((char *)s);
	ptrend = s_getend((char *)s);
	if (!(str = (char *)malloc((size_t)ptrend - (size_t)ptrbeg + 1)))
		return (NULL);
	begin = str;
	while (ptrbeg < ptrend)
		*str++ = *ptrbeg++;
	*str = '\0';
	return (begin);
}
