
# include "math.h"
# include "libft.h"
# include "memory_manager.h"

# define TEST_LENGTH	10000
# define MAX_ALLOC	4096
# define NB_TESTS	10000

struct			s_test {
	void		*ptr;
	uint8_t		c;
	size_t		size;
};

static int		add_sodo(
			struct s_test tab_ptr[TEST_LENGTH],
			int nb_elmt)
{
	int		i;

	i = rand(MAX_ALLOC - 1);
	tab_ptr[nb_elmt].c = i % 256;
	tab_ptr[nb_elmt].ptr = kmalloc(i);
	if (tab_ptr[nb_elmt].ptr == NULL) {
		ft_printf("%s: OUT OF MEMORY\n", __func__);
		return -1;
	}
	tab_ptr[nb_elmt].size = (size_t)i;
	ft_memset(tab_ptr[nb_elmt].ptr, tab_ptr[nb_elmt].c, i);
	return 0;
}

static int		del_sodo(
			struct s_test tab_ptr[TEST_LENGTH],
			int nb_elmt)
{
	int		i;
	size_t		n;
	uint8_t		*ptr;

	n = 0;
	i = rand(nb_elmt - 1);
	ptr = (uint8_t *)tab_ptr[i].ptr;
	while (n < tab_ptr[i].size)
	{
		if (*ptr != tab_ptr[i].c)
		{
			ft_printf("%s: BAD VALUE: Got %hhx instead of %hhx\n",
					__func__, *ptr, tab_ptr[i].c);
			return -1;
		}
		ptr++;
		n++;
	}
	kfree(tab_ptr[i].ptr);
	if (i != (nb_elmt - 1))
		tab_ptr[i] = tab_ptr[nb_elmt - 1];
	return 0;
}

static int		loop_sodo_test(
			struct s_test tab_ptr[TEST_LENGTH],
			int global_count[2],
			int *nb_elmt)
{
	int		op;
	int		i;
	int		max_alloc = 0;

	i = 0;
	while (i < NB_TESTS)
	{
		op = rand(2);
// XXX More allocation then free: 0, 1 => allocation, 2 => free
		if (*nb_elmt == 0
				|| (op < 2
				&& *nb_elmt < TEST_LENGTH))
		{
			if (add_sodo(tab_ptr, *nb_elmt) == -1)
				return -1;
			*nb_elmt += 1;
			if (*nb_elmt > max_alloc)
				max_alloc = *nb_elmt;
			global_count[0] += 1;
		}
		else
		{
			if (del_sodo(tab_ptr, *nb_elmt) == -1)
				return -1;
			*nb_elmt -= 1;
			global_count[1] += 1;
		}
		i++;
	}
	return max_alloc;
}

static int		real_sodo_next(
			struct s_test tab_ptr[TEST_LENGTH],
			size_t x,
			uint8_t *ptr,
			int i)
{
	size_t		n;
	size_t		n_size;

	if ((tab_ptr[i].ptr = krealloc(tab_ptr[i].ptr, x)) == NULL)
	{
		ft_printf("%s: OUT OF MEMORY\n", __func__);
		return -1;
	}
	n = 0;
	ptr = (uint8_t *)tab_ptr[i].ptr;
	n_size = (tab_ptr[i].size < x) ? tab_ptr[i].size : x;
	while (n < n_size)
	{
		if (*ptr != tab_ptr[i].c)
		{
			ft_printf("%s: BAD VALUE: Got %hhx instead of %hhx\n",
					__func__, *ptr, tab_ptr[i].c);
			return -1;
		}
		ptr++;
		n++;
	}
	tab_ptr[i].size = (size_t)x;
	tab_ptr[i].c = x % 256;
	ft_memset(tab_ptr[i].ptr, tab_ptr[i].c, x);
	return 0;
}

static int		real_sodo(
			struct s_test tab_ptr[TEST_LENGTH],
			int *nb_elmt)
{
	uint8_t		*ptr;
	size_t		n;
	size_t		x;
	int		i;

	n = 0;
	i = rand(*nb_elmt - 1);
	ptr = (uint8_t *)tab_ptr[i].ptr;
	while (n < tab_ptr[i].size)
	{
		if (*ptr != tab_ptr[i].c)
		{
			ft_printf("%s: BAD VALUE: Got %hhx instead of %hhx\n",
					__func__, *ptr, tab_ptr[i].c);
			return -1;
		}
		ptr++;
		n++;
	}
	x = rand(MAX_ALLOC - 1);
	if (ptr == NULL || x == 0)
		return 0;
	return real_sodo_next(tab_ptr, x, ptr, i);
}

static int		loop_sodo_realloc(
			struct s_test tab_ptr[TEST_LENGTH],
			int global_count[2],
			int *nb_elmt)
{
	int		op;
	int		i;
	int		max_alloc;

	i = -1;
	while (++i < NB_TESTS)
	{
		op = rand(2);
		if (*nb_elmt == 0 || (op == 0 && *nb_elmt < TEST_LENGTH))
		{
			if (add_sodo(tab_ptr, *nb_elmt) == -1)
				return -1;
			*nb_elmt += 1;
			if (*nb_elmt > max_alloc)
				max_alloc = *nb_elmt;
			global_count[0] += 1;
		}
		else if (op == 1)
		{
			if (del_sodo(tab_ptr, *nb_elmt) == -1)
				return -1;
			*nb_elmt -= 1;
			global_count[1] += 1;
		}
		else
		{
			if (real_sodo(tab_ptr, nb_elmt) == -1)
				return -1;
			global_count[2] += 1;
		}
	}
	return max_alloc;
}


static int		sodo_realloc(struct s_test tab_ptr[TEST_LENGTH])
{
	int		nb_elmt;
	int		global_count[3];
	int		i;
	int		max_alloc;

	nb_elmt = 0;
	ft_memset(global_count, 0, 3 * sizeof(int));
	if ((max_alloc = loop_sodo_realloc(
			tab_ptr, global_count, &nb_elmt)) == -1)
		return -1;
	kshow_alloc_mem();
	i = 0;
	if (nb_elmt != 0)
	{
		while (i < nb_elmt - 1)
		{
			kfree(tab_ptr[i].ptr);
			i++;
		}
	}
	kshow_alloc_mem();
	ft_printf("Max allocated blocks: %i\n", max_alloc);
	ft_printf("%i krealloc made, %i kmalloc and %i kfree made\n",
			global_count[2], global_count[0], global_count[1]);
	return 0;
}

static int		sodo_test(struct s_test	tab_ptr[TEST_LENGTH])
{
	int		nb_elmt;
	int		global_count[2];
	int		i;
	int		max_alloc;

	nb_elmt = 0;
	ft_memset(global_count, 0, 2 * sizeof(int));
	if ((max_alloc = loop_sodo_test
			(tab_ptr, global_count, &nb_elmt)) == -1)
		return -1;
	kshow_alloc_mem();
	ft_printf("nb elmt = %i\n", nb_elmt);
	i = 0;
	while (i < nb_elmt)
	{
		kfree(tab_ptr[i].ptr);
		i++;
	}
	kshow_alloc_mem();
	ft_printf("Max allocated blocks: %i\n", max_alloc);
	ft_printf("%i kmalloc made, %i kfree made\n",
			global_count[0], global_count[1]);
	return 0;
}

int			sodo(void)
{
	struct s_test	tab_ptr[TEST_LENGTH];

	srand(0xA8B0);
	if (sodo_test(tab_ptr) == -1)
		return -1;

	srand(0x15CF);
	if (sodo_realloc(tab_ptr) == -1)
		return -1;
	return 0;
}
