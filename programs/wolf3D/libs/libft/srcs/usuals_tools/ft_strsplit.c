/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   ft_strsplit.c                                      :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/04/10 17:01:52 by bmickael          #+#    #+#             */
/*   Updated: 2017/04/12 18:01:26 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include <stdlib.h>
#include "libft.h"

static char		**tab_words(const char *s, char c)
{
	int		i;
	int		t;
	int		n;
	char	**tab;

	i = 0;
	n = 0;
	t = 0;
	while (s[i])
	{
		if (t == 0 && s[i] != c)
		{
			t = 1;
			n = n + 1;
		}
		if (s[i] == c)
			t = 0;
		i++;
	}
	if (!(tab = (char **)malloc((n + 1) * sizeof(char *))))
		return (NULL);
	tab[n] = NULL;
	return (tab);
}

static int		l_num(const char *s, char c)
{
	int i;
	int l;

	i = 0;
	l = 0;
	while (s[i])
	{
		if (s[i] != c)
			l++;
		else
			return (l);
		i++;
	}
	return (l);
}

static char		**fflush_buff(char **tab, int w)
{
	int i;

	i = 0;
	while (i < w)
		free(tab[i++]);
	free(tab);
	return (NULL);
}

char			**ft_strsplit(char const *s, char c)
{
	char	**tab;
	int		w;
	int		i;
	int		j;

	if (!(tab = tab_words(s, c)))
		return (NULL);
	w = 0;
	while (*s)
	{
		if (*s == c)
			s++;
		else
		{
			i = 0;
			j = l_num(s, c);
			if (!(tab[w] = (char *)malloc((j + 1) * sizeof(char))))
				return (fflush_buff(tab, w));
			while (*s != c && *s != '\0')
				tab[w][i++] = *s++;
			tab[w++][i] = '\0';
		}
	}
	return (tab);
}
