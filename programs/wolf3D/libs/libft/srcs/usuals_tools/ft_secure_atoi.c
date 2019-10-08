/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   ft_secure_atoi.c                                   :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/04/10 16:25:29 by bmickael          #+#    #+#             */
/*   Updated: 2017/04/10 20:40:23 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include <limits.h>
#include "libft.h"

static int		secure_mul(int a, int b, int *error)
{
	if (a > INT_MAX / b)
		*error = TRUE;
	if ((a < INT_MIN / b))
		*error = TRUE;
	if ((a == -1) && (b == INT_MIN))
		*error = TRUE;
	if ((b == -1) && (a == INT_MIN))
		*error = TRUE;
	return (a * b);
}

static int		secure_add(int a, int b, int *error)
{
	if ((a > 0) && (b > INT_MAX - a))
		*error = TRUE;
	if ((a < 0) && (b < INT_MIN - a))
		*error = TRUE;
	return (a + b);
}

int				ft_secure_atoi(const char *nptr, int *error)
{
	int result;
	int sign;

	*error = FALSE;
	result = 0;
	sign = FALSE;
	if (*nptr == '-' || *nptr == '+')
	{
		if (*nptr == '-')
			sign = TRUE;
		nptr++;
	}
	if (!(*nptr >= '0' && *nptr <= '9'))
		*error = TRUE;
	else
		while (*nptr >= '0' && *nptr <= '9')
		{
			result = secure_mul(result, 10, error);
			result = secure_add(result, ((sign) ?
				-1 * (*nptr - '0') : (*nptr - '0')), error);
			nptr++;
		}
	if (*nptr != '\0')
		*error = TRUE;
	return (result);
}
