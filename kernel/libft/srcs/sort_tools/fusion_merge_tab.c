/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   fusion_merge_tab.c                                 :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/04/12 16:59:10 by bmickael          #+#    #+#             */
/*   Updated: 2017/04/12 16:59:15 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "sort_tools.h"

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

int				fusion_merge_tab(
		void ***t1,
		int len,
		int (*cmp)(void *, void *),
		struct s_custom_memory_fn *mem)
{
	void **t2;
	void **tmp;

	if (!cmp || !mem || !mem->allocator || !mem->deallocator || !t1)
		return (-EINVAL);
	if (!len)
		return (0);
	if (!(t2 = (void **)mem->allocator(len * sizeof(void *))))
		return (-1);
	if ((tmp = exec(*t1, t2, len, cmp)) == *t1)
		mem->deallocator(t2);
	else
	{
		mem->deallocator(*t1);
		*t1 = tmp;
	}
	return (len);
}
