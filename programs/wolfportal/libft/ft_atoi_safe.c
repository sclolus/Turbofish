/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   ft_atoi_safe.c                                     :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: vcombey <marvin@42.fr>                     +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/04/28 10:49:13 by vcombey           #+#    #+#             */
/*   Updated: 2017/04/28 10:49:31 by vcombey          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "libft.h"

static int	set_result(int *result)
{
	*result = -2147483648;
	return (1);
}

int			ft_atoi_safe(const char *str, int *result)
{
	int	sign;

	*result = 0;
	sign = 0;
	while (*str == ' ' || ('\t' <= *str && *str <= '\r'))
		str++;
	if (ft_strcmp(str, "-2147483648") == 0)
		return (set_result(result));
	if (*str == '+' || *str == '-')
	{
		sign = (*str == '-' ? -1 : 1);
		str++;
	}
	if (*str == '\0')
		return (0);
	while ('0' <= *str && *str <= '9')
	{
		*result = (*result * 10) + (*str - '0');
		str++;
	}
	*result = sign == -1 ? -(*result) : *result;
	if (*str != '\0')
		return (0);
	return (1);
}
