/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   ft_itoa.c                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/04/10 17:03:58 by bmickael          #+#    #+#             */
/*   Updated: 2017/04/10 17:05:13 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include <stdlib.h>
#include "libft.h"

char	*ft_itoa(int n)
{
	char	*output;
	int		string_size;
	int		i;
	int		sign;

	sign = (n < 0) ? 1 : 0;
	string_size = 1;
	i = n;
	while ((i = i / 10))
		string_size++;
	if (!(output = (char *)malloc((string_size + sign + 1) * sizeof(char))))
		return (NULL);
	output[string_size + sign] = '\0';
	if (sign)
		output[0] = '-';
	i = string_size + sign - 1;
	while (i != (-1 + sign))
	{
		output[i--] = (sign) ? HEX_T(-(n % 10)) : HEX_T((n % 10));
		n /= 10;
	}
	return (output);
}
