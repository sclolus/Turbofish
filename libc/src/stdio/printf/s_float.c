#include "internal_printf.h"

/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   ft_putstr.c                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/04/10 17:07:49 by bmickael          #+#    #+#             */
/*   Updated: 2017/04/10 17:08:15 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include <unistd.h>
#include <string.h>

void	ft_putstr(const char *s)
{
	write(1, s, strlen(s));
}

/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   itoa.c                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/04/10 17:03:58 by bmickael          #+#    #+#             */
/*   Updated: 2017/04/10 17:05:13 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include <stdlib.h>

# define HEX_T(x)	"0123456789ABCDEF"[x]

char	*itoa(int n)
{
	char *output;
	int string_size;
	int i;
	int sign;

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

#warning PRINTF/FLOAT IMPLEMENTATION LACKS 90% OF NEEDED FEATURES

int s_float(t_args *args, t_status *op)
{
	double n;

	if (args->l & L) {
		n = va_arg(op->ap, double);
	} else {
		n = (double)va_arg(op->ap, double);
	}
	// We only display the integer part
	char *s = itoa((int)n);
	ft_putstr(s);
	free(s);
	return (0);
}
