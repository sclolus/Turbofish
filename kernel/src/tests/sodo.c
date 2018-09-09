
# include "libft.h"

void			srand(u32 seed) {
	(void)seed;
}

int			rand(void) {
	return 0;
}

void			*kmalloc(size_t size);
void			kfree(void *ptr);
void			kshow_alloc_mem(void);
void			kshow_alloc_mem_ex(void);
void			*krealloc(void *ptr, size_t size);

# define TEST_LENGTH	100000
# define MAX_ALLOC	1024
# define NB_TESTS	10000000

struct			s_test {
	void		*ptr;
	uint8_t		c;
	size_t		size;
};

void			add_sodo(
			struct s_test tab_ptr[TEST_LENGTH],
			int nb_elmt);

void			del_sodo(
			struct s_test tab_ptr[TEST_LENGTH],
			int nb_elmt);

void			real_sodo(
			struct s_test tab_ptr[TEST_LENGTH],
			int *nb_elmt);

static void		loop_sodo_test(
			struct s_test tab_ptr[TEST_LENGTH],
			int global_count[2],
			int *nb_elmt)
{
	int		op;
	int		i;

	srand(0xAABBCCDD);
	i = 0;
	while (i < NB_TESTS)
	{
		op = rand();
		if (*nb_elmt == 0 || ((op & 0x1) == 0 && *nb_elmt < TEST_LENGTH))
		{
			add_sodo(tab_ptr, *nb_elmt);
			*nb_elmt += 1;
			global_count[0] += 1;
		}
		else
		{
			del_sodo(tab_ptr, *nb_elmt);
			*nb_elmt -= 1;
			global_count[1] += 1;
		}
		i++;
	}
}

void			sodo_test(struct s_test	tab_ptr[TEST_LENGTH])
{
	int		nb_elmt;
	int		global_count[2];
	int		i;

	srand(0x1A2B3C4D);
	nb_elmt = 0;
	ft_memset(global_count, 0, 2 * sizeof(int));
	loop_sodo_test(tab_ptr, global_count, &nb_elmt);
	ft_printf("%i malloc made,  %i free made\n",
			global_count[0], global_count[1]);
	kshow_alloc_mem();
	i = 0;
	while (i < nb_elmt)
	{
		kfree(tab_ptr[i].ptr);
		i++;
	}
	kshow_alloc_mem();
}

static void		loop_sodo_realloc(
			struct s_test tab_ptr[TEST_LENGTH],
			int global_count[2],
			int *nb_elmt)
{
	int		op;
	int		i;

	i = -1;
	while (++i < NB_TESTS)
	{
		op = rand() % 3;
		if (*nb_elmt == 0 || (op == 0 && *nb_elmt < TEST_LENGTH))
		{
			add_sodo(tab_ptr, *nb_elmt);
			*nb_elmt += 1;
			global_count[0] += 1;
		}
		else if (op == 1)
		{
			del_sodo(tab_ptr, *nb_elmt);
			*nb_elmt -= 1;
			global_count[1] += 1;
		}
		else
		{
			real_sodo(tab_ptr, nb_elmt);
			global_count[2] += 1;
		}
	}
}

void			sodo_realloc(struct s_test tab_ptr[TEST_LENGTH])
{
	int		nb_elmt;
	int		global_count[3];
	int		i;

	srand(0x12345678);
	nb_elmt = 0;
	ft_memset(global_count, 0, 3 * sizeof(int));
	loop_sodo_realloc(tab_ptr, global_count, &nb_elmt);
	ft_printf("%i realloc made, %i mallocs and %i free made\n",
			global_count[2], global_count[0], global_count[1]);
	kshow_alloc_mem_ex();
	i = 0;
	if (nb_elmt != 0)
	{
		while (i < nb_elmt - 1)
		{
			kfree(tab_ptr[i].ptr);
			i++;
		}
	}
	kshow_alloc_mem_ex();
}

void			add_sodo(
			struct s_test tab_ptr[TEST_LENGTH],
			int nb_elmt)
{
	int		i;

	i = rand() % (MAX_ALLOC);
	tab_ptr[nb_elmt].c = i % 256;
	tab_ptr[nb_elmt].ptr = kmalloc(i);
	tab_ptr[nb_elmt].size = (size_t)i;
	ft_memset(tab_ptr[nb_elmt].ptr, tab_ptr[nb_elmt].c, i);
}

void			del_sodo(
			struct s_test tab_ptr[TEST_LENGTH],
			int nb_elmt)
{
	int		i;
	size_t		n;
	uint8_t		*ptr;

	n = 0;
	i = rand() % nb_elmt;
	ptr = (uint8_t *)tab_ptr[i].ptr;
	while (n < tab_ptr[i].size)
	{
		if (*ptr != tab_ptr[i].c)
		{
			ft_printf("BAD VALUE: Got %hhx instead of %hhx\n",
					*ptr, tab_ptr[i].c);
			while (1);
		}
		ptr++;
		n++;
	}
	kfree(tab_ptr[i].ptr);
	if (i != (nb_elmt - 1))
		tab_ptr[i] = tab_ptr[nb_elmt - 1];
}

static void		real_sodo_next(
			struct s_test tab_ptr[TEST_LENGTH],
			size_t x,
			uint8_t *ptr,
			int i)
{
	size_t		n;
	size_t		n_size;

	if ((tab_ptr[i].ptr = krealloc(tab_ptr[i].ptr, x)) == NULL)
	{
		ft_printf("BAD REALLOC\n");
		while (1);
	}
	n = 0;
	ptr = (uint8_t *)tab_ptr[i].ptr;
	n_size = (tab_ptr[i].size < x) ? tab_ptr[i].size : x;
	while (n < n_size)
	{
		if (*ptr != tab_ptr[i].c)
		{
			ft_printf("BAD VALUE: Got %hhx instead of %hhx\n",
					*ptr, tab_ptr[i].c);
			while (1);
		}
		ptr++;
		n++;
	}
	tab_ptr[i].size = (size_t)x;
	tab_ptr[i].c = x % 256;
	ft_memset(tab_ptr[i].ptr, tab_ptr[i].c, x);
}

void			real_sodo(
			struct s_test tab_ptr[TEST_LENGTH],
			int *nb_elmt)
{
	uint8_t		*ptr;
	size_t		n;
	size_t		x;
	int		i;

	n = 0;
	i = rand() % *nb_elmt;
	ptr = (uint8_t *)tab_ptr[i].ptr;
	while (n < tab_ptr[i].size)
	{
		if (*ptr != tab_ptr[i].c)
		{
			ft_printf("BAD VALUE: Got %hhx instead of %hhx\n",
					*ptr, tab_ptr[i].c);
			while (1);
		}
		ptr++;
		n++;
	}
	x = rand() % (MAX_ALLOC);
	if (ptr == NULL || x == 0)
		return ;
	real_sodo_next(tab_ptr, x, ptr, i);
}

int			sodo(void)
{
	struct s_test	tab_ptr[TEST_LENGTH];

	sodo_test(tab_ptr);
	sodo_realloc(tab_ptr);
	return 0;
}
