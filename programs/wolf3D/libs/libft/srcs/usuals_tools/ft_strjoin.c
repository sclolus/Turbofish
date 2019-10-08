/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   ft_strjoin.c                                       :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/04/10 16:50:07 by bmickael          #+#    #+#             */
/*   Updated: 2017/04/10 16:51:21 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include <stdlib.h>
#include "libft.h"

char	*ft_strjoin(char const *s1, char const *s2)
{
	char	*output;
	int		i;
	int		j;

	if ((output = (char *)
		malloc((ft_strlen(s1) + ft_strlen(s2) + 1) * sizeof(char))))
	{
		i = 0;
		while (s1[i])
		{
			output[i] = s1[i];
			i++;
		}
		j = 0;
		while (s2[j])
			output[i++] = s2[j++];
		output[i] = '\0';
		return (output);
	}
	return (NULL);
}
