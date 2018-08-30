/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   fusion_merge_chain_list.c                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/04/12 16:59:29 by bmickael          #+#    #+#             */
/*   Updated: 2017/04/12 16:59:33 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "sort_tools.h"

static void		fill_from_tab(struct s_list *lst, void **t, int limit)
{
	int i;

	i = 0;
	while (i < limit)
	{
		lst->content = t[i++];
		lst = lst->next;
	}
}

static void		merge_mod(void **s1, void **s2, void **end, struct s_info *w)
{
	void **p_gr_1;
	void **p_gr_2;

	while ((p_gr_1 = s1) < end)
	{
		p_gr_2 = p_gr_1 + w->offset;
		while (TRUE)
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

static void		**exec(void **t1, void **t2, int l, int (*cmp)(void *, void *))
{
	struct s_info	w;
	int				state;

	w.cmp = cmp;
	w.offset = 1;
	state = FALSE;
	while (w.offset < l)
	{
		if (state == FALSE)
			merge_mod(t1, t2, t1 + l, &w);
		else
			merge_mod(t2, t1, t2 + l, &w);
		state = (state) ? FALSE : TRUE;
		w.offset <<= 1;
	}
	return ((state) ? t2 : t1);
}

static int		multiple_alloc(
		void ***t1,
		void ***t2,
		int len,
		struct s_custom_memory_fn *mem)
{
	if (!(*t1 = (void **)mem->allocator(len * sizeof(void *))))
		return (0);
	if (!(*t2 = (void **)mem->allocator(len * sizeof(void *))))
	{
		mem->deallocator(*t1);
		return (0);
	}
	(void)len;
	return (1);
}

int				fusion_merge_chain_list(
		struct s_list *lst,
		int (*cmp)(void *, void *),
		struct s_custom_memory_fn *mem)
{
	struct s_list	*origin;
	void			**t1;
	void			**t2;
	int				len;
	int				i;

	if (!mem || !cmp || !mem->allocator || !mem->deallocator)
		return (-EINVAL);
	if (lst == NULL)
		return (0);
	origin = lst;
	len = 1;
	while ((lst = lst->next))
		len++;
	if (!(multiple_alloc(&t1, &t2, len, mem)))
		return (-1);
	i = 0;
	lst = origin;
	t1[i++] = lst->content;
	while ((lst = lst->next))
		t1[i++] = lst->content;
	fill_from_tab(origin, exec(t1, t2, len, cmp), len);
	mem->deallocator(t1);
	mem->deallocator(t2);
	return (len);
}
