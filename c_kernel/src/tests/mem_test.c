
# include "dynamic_allocator.h"
# include "../memory/memory_manager.h"

# include "math.h"
# include "libft.h"
# include "chained_tools.h"
# include "tests.h"

# define TEST_LENGTH	100

struct			s_test {
	void		*ptr;
	uint8_t		c;
	size_t		size;
};

struct			mem_test_ctx {
	struct s_test	tab_ptr[TEST_LENGTH];
	u32		nb_tests;
	u32		max_alloc;

	struct s_list	*log;
	char		*err_ptr;
};

/*
 * Log functions
 */
enum			mem_status {
	allocated = 0,
	deallocated,
	reallocated
};

struct			mem_log_entry {
	void		*virt_addr;
	char		c;
	size_t		size;
	enum mem_status status;
};

static void		del_log(void *content, size_t size, int (*free)(void *))
{
	int ret;

	(void)size;
	ret = free(content);
	if (ret < 0) {
		eprintk("internal log error\n");
		return ;
	}
}

static void		add_log_entry(
			struct mem_test_ctx *ctx,
			void *virt_addr,
			char c,
			size_t size,
			enum mem_status status)
{
	struct mem_log_entry *log_entry;

	log_entry = (struct mem_log_entry *)
			kmalloc(sizeof(struct mem_log_entry));
	if (log_entry == NULL) {
		eprintk("internal log error\n");
		return ;
	}
	log_entry->c = c;
	log_entry->size = size;
	log_entry->virt_addr = virt_addr;
	log_entry->status = status;
	lst_push_front(&ctx->log, log_entry, sizeof(*log_entry), &kmalloc);
}

static void		dump_log(struct s_list *lst)
{
	struct mem_log_entry *log_entry;

	log_entry = (struct mem_log_entry *)lst->content;
	if (log_entry == NULL) {
		eprintk("internal log error\n");
		return ;
	}

	printk("%s -> %p %.2hhx size: %u\n",
			((log_entry->status == allocated) ? "PUSH" : "POP "),
			log_entry->virt_addr,
			log_entry->c,
			log_entry->size);
}

/*
 * MEM TEST main functions
 */

static int		add_object(
			struct mem_test_ctx *ctx,
			int nb_elmt,
			void *(*allocator)(size_t))
{
	u32 size = (u32)rand(ctx->max_alloc - 1);
	ctx->tab_ptr[nb_elmt].c = size % 256;

	if (size == 0)
		size = 1;

	ctx->tab_ptr[nb_elmt].ptr = allocator(size);
	if (ctx->tab_ptr[nb_elmt].ptr == NULL) {
		printk("%s: OUT OF MEMORY\n", __func__);
		return -1;
	}
	ctx->tab_ptr[nb_elmt].size = size;
	add_log_entry(
			ctx,
			ctx->tab_ptr[nb_elmt].ptr,
			ctx->tab_ptr[nb_elmt].c,
			size,
			allocated);
	ft_memset(ctx->tab_ptr[nb_elmt].ptr, ctx->tab_ptr[nb_elmt].c, size);
	return 0;
}

static int		del_object(
			struct mem_test_ctx *ctx,
			int nb_elmt,
			int (*deallocator)(void *))
{
	int		i;
	size_t		n;
	uint8_t		*ptr;

	n = 0;
	i = rand(nb_elmt - 1);
	ptr = (uint8_t *)ctx->tab_ptr[i].ptr;
	add_log_entry(
			ctx,
			ctx->tab_ptr[i].ptr,
			ctx->tab_ptr[i].c,
			ctx->tab_ptr[i].size,
			deallocated);
	while (n < ctx->tab_ptr[i].size) {
		if (*ptr != ctx->tab_ptr[i].c) {
			ctx->err_ptr = (char *)ptr;
			return -1;
		}
		ptr++;
		n++;
	}
	deallocator(ctx->tab_ptr[i].ptr);
	if (i != (nb_elmt - 1))
		ctx->tab_ptr[i] = ctx->tab_ptr[nb_elmt - 1];
	return 0;
}

static int		loop_test(
			struct mem_test_ctx *ctx,
			int global_count[2],
			int *nb_elmt,
			void *(*allocator)(size_t),
			int (*deallocator)(void *))
{
	u32		i;
	int		op;
	int		max_alloc = 0;

	i = 0;
	while (i < ctx->nb_tests) {
		op = rand(2);
// XXX More allocation then free: 0, 1 => allocation, 2 => free
		if (*nb_elmt == 0
				|| (op < 2
				&& *nb_elmt < TEST_LENGTH)) {
			if (add_object(ctx, *nb_elmt, allocator) == -1)
				return -1;
			*nb_elmt += 1;
			if (*nb_elmt > max_alloc)
				max_alloc = *nb_elmt;
			global_count[0] += 1;
		}
		else {
			if (del_object(ctx, *nb_elmt, deallocator) == -1)
				return -1;
			*nb_elmt -= 1;
			global_count[1] += 1;
		}
		i++;
	}
	return max_alloc;
}

static int		realloc_test_next(
			struct mem_test_ctx *ctx,
			size_t x,
			uint8_t *ptr,
			int i)
{
	size_t		n;
	size_t		n_size;

	if ((ctx->tab_ptr[i].ptr = krealloc(ctx->tab_ptr[i].ptr, x)) == NULL) {
		printk("%s: OUT OF MEMORY\n", __func__);
		return -1;
	}
	n = 0;
	ptr = (uint8_t *)ctx->tab_ptr[i].ptr;
	n_size = (ctx->tab_ptr[i].size < x) ? ctx->tab_ptr[i].size : x;
	while (n < n_size) {
		if (*ptr != ctx->tab_ptr[i].c) {
			ctx->err_ptr = (char *)ptr;
			return -1;
		}
		ptr++;
		n++;
	}
	ctx->tab_ptr[i].size = (size_t)x;
	ctx->tab_ptr[i].c = x % 256;
	ft_memset(ctx->tab_ptr[i].ptr, ctx->tab_ptr[i].c, x);
	return 0;
}

static int		realloc_test(
			struct mem_test_ctx *ctx,
			int *nb_elmt)
{
	uint8_t		*ptr;
	size_t		n;
	size_t		size;
	int		i;

	n = 0;
	i = rand(*nb_elmt - 1);
	ptr = (uint8_t *)ctx->tab_ptr[i].ptr;

	add_log_entry(
			ctx,
			ctx->tab_ptr[i].ptr,
			ctx->tab_ptr[i].c,
			ctx->tab_ptr[i].size,
			reallocated);

	while (n < ctx->tab_ptr[i].size) {
		if (*ptr != ctx->tab_ptr[i].c) {
			ctx->err_ptr = (char *)ptr;
			return -1;
		}
		ptr++;
		n++;
	}
	size = (u32)rand(ctx->max_alloc - 1);
	if (ptr == NULL || size == 0)
		return 0;
	return realloc_test_next(ctx, size, ptr, i);
}

static int		loop_realloc(
			struct mem_test_ctx *ctx,
			int global_count[2],
			int *nb_elmt)
{
	u32		i;
	int		op;
	int		max_alloc;

	max_alloc = 0;
	i = 0;
	while (i < ctx->nb_tests) {
		op = rand(2);
		if (*nb_elmt == 0 || (op == 0 && *nb_elmt < TEST_LENGTH)) {
			if (add_object(ctx, *nb_elmt, &kmalloc) == -1)
				return -1;
			*nb_elmt += 1;
			if (*nb_elmt > max_alloc)
				max_alloc = *nb_elmt;
			global_count[0] += 1;
		} else if (op == 1) {
			if (del_object(ctx, *nb_elmt, &kfree) == -1)
				return -1;
			*nb_elmt -= 1;
			global_count[1] += 1;
		} else {
			if (realloc_test(ctx, nb_elmt) == -1)
				return -1;
			global_count[2] += 1;
		}
		i++;
	}
	return max_alloc;
}


static int		base_realloc(struct mem_test_ctx *ctx, int verbosity)
{
	int		nb_elmt;
	int		global_count[3];
	int		i;
	int		max_alloc;

	nb_elmt = 0;
	ft_memset(global_count, 0, 3 * sizeof(int));
	if ((max_alloc = loop_realloc(
			ctx, global_count, &nb_elmt)) == -1)
		return -1;
	i = 0;
	if (nb_elmt != 0) {
		while (i < nb_elmt - 1) {
			kfree(ctx->tab_ptr[i].ptr);
			i++;
		}
	}
	if (verbosity) {
		printk("Max allocated blocks: %i\n", max_alloc);
		printk("%i krealloc made, %i kmalloc and %i kfree made\n",
				global_count[2],
				global_count[0],
				global_count[1]);
	}
	return 0;
}

static int		base_test(
			struct mem_test_ctx *ctx,
			void *(*allocator)(size_t),
			int (*deallocator)(void *),
			int verbosity)
{
	int		nb_elmt;
	int		global_count[2];
	int		i;
	int		max_alloc;

	nb_elmt = 0;
	ft_memset(global_count, 0, 2 * sizeof(int));
	if ((max_alloc = loop_test(
			ctx,
			global_count,
			&nb_elmt,
			allocator,
			deallocator)) == -1)
		return -1;
	if (verbosity)
		printk("nb elmt = %i\n", nb_elmt);
	i = 0;
	while (i < nb_elmt) {
		deallocator(ctx->tab_ptr[i].ptr);
		i++;
	}
	if (verbosity) {
		printk("Max allocated blocks: %i\n", max_alloc);
		printk("%i allocations made, %i deallocations made\n",
				global_count[0], global_count[1]);
	}
	return 0;
}

int			mem_test(enum mem_test_type type, int verbosity)
{
	struct mem_test_ctx	ctx;
	int			ret;

	bzero(&ctx.tab_ptr, TEST_LENGTH * sizeof(struct s_test));
	ctx.nb_tests = 10000;
	ctx.max_alloc = PAGE_SIZE * 16;
	ctx.log = NULL;
	srand(0xCDE1);

	switch (type) {
	case k_family:
		printk("K map memory group check: ");
		ret = base_test(&ctx, &kmmap, &kmunmap, verbosity);
		break;
	case v_family:
		printk("V map memory group check: ");
		ret = base_test(&ctx, &valloc, &vfree, verbosity);
		break;
	case k_sub_family:
		printk("K sub family check: ");
		ctx.max_alloc = PAGE_SIZE;
		ret = base_realloc(&ctx, verbosity);
		break;
	default:
		eprintk("%s: default case\n", __func__);
		return -1;
	}

	if (ret < 0) {
		printk("{red}FAIL\n{eoc}");

		if (verbosity) {
			struct mem_log_entry *last_entry =
					(struct mem_log_entry *)
					ctx.log->content;
			printk("BAD VALUE: Got %hhx instead of %hhx at %p\n",
					*(ctx.err_ptr),
					last_entry->c,
					ctx.err_ptr);
			lst_iter(ctx.log, &dump_log);
			get_anotomie_of(
					last_entry->virt_addr,
					last_entry->size);
		}
	} else {
		printk("{green}OK\n{eoc}");
	}
	lst_del(&ctx.log, &del_log, &kfree);
	return ret;
}
