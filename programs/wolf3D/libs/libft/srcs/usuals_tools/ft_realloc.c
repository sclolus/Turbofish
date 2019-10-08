/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   ft_realloc.c                                       :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/04/17 10:18:16 by bmickael          #+#    #+#             */
/*   Updated: 2017/04/17 11:29:22 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include <stdlib.h>
#include "libft.h"

void	*ft_realloc(void **input, size_t o_size, size_t n_size)
{
	void *output;

	if (!(output = ft_memalloc(n_size)))
	{
		if (*input)
			free(input);
		return (NULL);
	}
	ft_memcpy(output, *input, o_size);
	free(*input);
	*input = output;
	return (output);
}
