/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   ft_strdup.c                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: vcombey <marvin@42.fr>                     +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2016/11/10 03:17:29 by vcombey           #+#    #+#             */
/*   Updated: 2016/11/25 17:53:48 by vcombey          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "libft.h"

char	*ft_strdup(char const *s)
{
	char			*d;
	unsigned int	i;

	i = 0;
	if (!(d = ft_strnew(ft_strlen(s))))
		return (NULL);
	while (s[i])
	{
		d[i] = s[i];
		i++;
	}
	return (d);
}
