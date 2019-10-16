
#ifndef __COMMON_H__
# define __COMMON_H__

#include <string.h>

typedef struct s_list
{
	void *content;
	size_t content_size;
	struct s_list *next;
} t_list;

t_list *ft_lst_invert_it(t_list **alst);
t_list *ft_lst_push_front(t_list **alst, void *data, size_t len);

char *ft_itoa(int n);
int ft_merge_tab(void ***t1, int len, int (*cmp)(void *, void *));

typedef struct s_info
{
	int offset;
	int (*cmp)(void *, void *);
} t_info;

#endif
