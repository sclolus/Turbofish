
#include "common.h"

#include <stdlib.h>
#include <strings.h>
#include <stdbool.h>

# define HEX_T(x)	"0123456789ABCDEF"[x]

char	*ft_itoa(int n)
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

static void merge_mod(void **s1, void **s2, void **end, t_info *w)
{
	void **p_gr_1;
	void **p_gr_2;

	while ((p_gr_1 = s1) < end)
	{
		p_gr_2 = p_gr_1 + w->offset;
		while (true)
		{
			if (p_gr_2 < end)
				*s2++ = w->cmp(*p_gr_2, *p_gr_1) ? *p_gr_1++ : *p_gr_2++;
			if (p_gr_1 == (s1 + w->offset))
			{
				while (p_gr_2 != (s1 + (2 * w->offset)) && p_gr_2 < end)
					*s2++ = *p_gr_2++;
				break ;
			}
			else if (p_gr_2 == (s1 + (2 * w->offset)) || p_gr_2 >= end)
			{
				while (p_gr_1 != (s1 + w->offset) && p_gr_1 < end)
					*s2++ = *p_gr_1++;
				break ;
			}
		}
		s1 += 2 * w->offset;
	}
}

static void **exec(void **t1, void **t2, int l, int (*cmp)(void *, void *))
{
	t_info w;
	int state;

	bzero(&w, sizeof(t_info));
	w.cmp = cmp;
	w.offset = 1;
	state = false;
	while (w.offset < l)
	{
		if (state == false)
			merge_mod(t1, t2, t1 + l, &w);
		else
			merge_mod(t2, t1, t2 + l, &w);
		state = (state) ? false : true;
		w.offset <<= 1;
	}
	return ((state) ? t2 : t1);
}

int ft_merge_tab(void ***t1, int len, int (*cmp)(void *, void *))
{
	void **t2;
	void **tmp;

	if (!len)
		return (0);
	if (!(t2 = (void **)malloc(len * sizeof(void *))))
		return (-1);
	if ((tmp = exec(*t1, t2, len, cmp)) == *t1)
		free(t2);
	else
	{
		free(*t1);
		*t1 = tmp;
	}
	return (len);
}

t_list *ft_lst_create_elem(void *data, size_t len)
{
	t_list *elmt;

	if (!(elmt = (t_list *)malloc(sizeof(t_list))))
		return (NULL);
	elmt->content = data;
	elmt->content_size = len;
	return (elmt);
}

t_list *ft_lst_push_front(t_list **alst, void *data, size_t len)
{
	t_list *m;

	if (!(m = ft_lst_create_elem(data, len)))
		return (NULL);
	if (!(*alst))
	{
		*alst = m;
		m->next = NULL;
		return (*alst);
	}
	m->next = *alst;
	*alst = m;
	return (*alst);
}

t_list *ft_lst_invert_it(t_list **alst)
{
	t_list *p;
	t_list *c;
	t_list *n;

	p = NULL;
	c = *alst;
	while (c)
	{
		n = c->next;
		c->next = p;
		p = c;
		c = n;
	}
	*alst = p;
	return (*alst);
}
