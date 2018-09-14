
# include "math.h"
# include "libft.h"
# include "chained_tools.h"
# include "memory_manager.h"

# define TEST_LENGTH	100

struct			s_test {
	void		*ptr;
	uint8_t		c;
	size_t		size;
};

struct			sodo_ctx {
	struct s_test	tab_ptr[TEST_LENGTH];
	u32		nb_tests;
	u32		max_alloc;
	struct s_list	*log;
};

/*
 * Log functions
 */
enum			mem_status {
	allocated = 0,
	deallocated
};

struct			sodo_log_entry {
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
			struct sodo_ctx *ctx,
			void *virt_addr,
			char c,
			size_t size,
			enum mem_status status)
{
	struct sodo_log_entry *log_entry;

	log_entry = (struct sodo_log_entry *)
			kmalloc(sizeof(struct sodo_log_entry));
	if (log_entry == NULL) {
		eprintk("internal log error\n");
		return ;
	}
	log_entry->c = c;
	log_entry->size = size;
	log_entry->virt_addr = virt_addr;
	log_entry->status = status;
	lst_push_back(&ctx->log, log_entry, sizeof(*log_entry), &kmalloc);
}

static void		dump_log(struct s_list *lst)
{
	struct sodo_log_entry *log_entry;

	log_entry = (struct sodo_log_entry *)lst->content;
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

static int		add_sodo(
			struct sodo_ctx *ctx,
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
	memset(ctx->tab_ptr[nb_elmt].ptr, ctx->tab_ptr[nb_elmt].c, size);
	return 0;
}

static int		del_sodo(
			struct sodo_ctx *ctx,
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
			printk("%s: BAD VALUE: Got %hhx instead of %hhx\n",
					__func__, *ptr, ctx->tab_ptr[i].c);
			lst_iter(ctx->log, &dump_log);
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

static int		loop_sodo_test(
			struct sodo_ctx *ctx,
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
			if (add_sodo(ctx, *nb_elmt, allocator) == -1)
				return -1;
			*nb_elmt += 1;
			if (*nb_elmt > max_alloc)
				max_alloc = *nb_elmt;
			global_count[0] += 1;
		}
		else {
			if (del_sodo(ctx, *nb_elmt, deallocator) == -1)
				return -1;
			*nb_elmt -= 1;
			global_count[1] += 1;
		}
		i++;
	}
	return max_alloc;
}

static int		real_sodo_next(
			struct sodo_ctx *ctx,
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
			printk("%s: BAD VALUE: Got %hhx instead of %hhx\n",
					__func__, *ptr, ctx->tab_ptr[i].c);
			return -1;
		}
		ptr++;
		n++;
	}
	ctx->tab_ptr[i].size = (size_t)x;
	ctx->tab_ptr[i].c = x % 256;
	memset(ctx->tab_ptr[i].ptr, ctx->tab_ptr[i].c, x);
	return 0;
}

static int		real_sodo(
			struct sodo_ctx *ctx,
			int *nb_elmt)
{
	uint8_t		*ptr;
	size_t		n;
	size_t		size;
	int		i;

	n = 0;
	i = rand(*nb_elmt - 1);
	ptr = (uint8_t *)ctx->tab_ptr[i].ptr;
	while (n < ctx->tab_ptr[i].size) {
		if (*ptr != ctx->tab_ptr[i].c) {
			printk("%s: BAD VALUE: Got %hhx instead of %hhx\n",
					__func__, *ptr, ctx->tab_ptr[i].c);
			return -1;
		}
		ptr++;
		n++;
	}
	size = (u32)rand(ctx->max_alloc - 1);
	if (ptr == NULL || size == 0)
		return 0;
	return real_sodo_next(ctx, size, ptr, i);
}

static int		loop_sodo_realloc(
			struct sodo_ctx *ctx,
			int global_count[2],
			int *nb_elmt)
{
	u32		i;
	int		op;
	int		max_alloc;

	i = 0;
	while (i < ctx->nb_tests) {
		op = rand(2);
		if (*nb_elmt == 0 || (op == 0 && *nb_elmt < TEST_LENGTH)) {
			if (add_sodo(ctx, *nb_elmt, &kmalloc) == -1)
				return -1;
			*nb_elmt += 1;
			if (*nb_elmt > max_alloc)
				max_alloc = *nb_elmt;
			global_count[0] += 1;
		} else if (op == 1) {
			if (del_sodo(ctx, *nb_elmt, &kfree) == -1)
				return -1;
			*nb_elmt -= 1;
			global_count[1] += 1;
		} else {
			if (real_sodo(ctx, nb_elmt) == -1)
				return -1;
			global_count[2] += 1;
		}
		i++;
	}
	return max_alloc;
}


static int		sodo_realloc(struct sodo_ctx *ctx)
{
	int		nb_elmt;
	int		global_count[3];
	int		i;
	int		max_alloc;

	nb_elmt = 0;
	memset(global_count, 0, 3 * sizeof(int));
	if ((max_alloc = loop_sodo_realloc(
			ctx, global_count, &nb_elmt)) == -1)
		return -1;
	i = 0;
	if (nb_elmt != 0) {
		while (i < nb_elmt - 1) {
			kfree(ctx->tab_ptr[i].ptr);
			i++;
		}
	}
	printk("Max allocated blocks: %i\n", max_alloc);
	printk("%i krealloc made, %i kmalloc and %i kfree made\n",
			global_count[2], global_count[0], global_count[1]);
	return 0;
}

static int		sodo_test(
			struct sodo_ctx *ctx,
			void *(*allocator)(size_t),
			int (*deallocator)(void *))
{
	int		nb_elmt;
	int		global_count[2];
	int		i;
	int		max_alloc;

	nb_elmt = 0;
	memset(global_count, 0, 2 * sizeof(int));
	if ((max_alloc = loop_sodo_test(
			ctx,
			global_count,
			&nb_elmt,
			allocator,
			deallocator)) == -1)
		return -1;
	printk("nb elmt = %i\n", nb_elmt);
	i = 0;
	while (i < nb_elmt) {
		deallocator(ctx->tab_ptr[i].ptr);
		i++;
	}
	printk("Max allocated blocks: %i\n", max_alloc);
	printk("%i allocations made, %i deallocations made\n",
			global_count[0], global_count[1]);
	return 0;
}

int			sodo(void)
{
	struct sodo_ctx ctx;

	printk("\n");
	printk("{orange}K map memory group check:{eoc}\n");
	bzero(&ctx.tab_ptr, TEST_LENGTH * sizeof(struct s_test));
	ctx.nb_tests = 10000;
	ctx.max_alloc = 4096 * 4;
	ctx.log = NULL;
	srand(0xCDE1);
	if (sodo_test(&ctx, &kmmap, &kmunmap) == -1) {
	//	return -1;
	}
	lst_del(&ctx.log, &del_log, &kfree);

	printk("\n");
	printk("{orange}V map memory group check:{eoc}\n");
	bzero(&ctx.tab_ptr, TEST_LENGTH * sizeof(struct s_test));
	ctx.nb_tests = 10000;
	ctx.max_alloc = 4096 * 4;
	ctx.log = NULL;
	srand(0xA8B0);
	if (sodo_test(&ctx, &valloc, &vfree) == -1) {
	//	return -1;
	}
	lst_del(&ctx.log, &del_log, &kfree);

	printk("\n");
	printk("{orange}K sub family check:{eoc}\n");
	ctx.nb_tests = 10000;
	ctx.max_alloc = 4096;
	bzero(&ctx.tab_ptr, TEST_LENGTH * sizeof(struct s_test));
	srand(0x15CF);
	if (sodo_realloc(&ctx) == -1) {
	//	return -1;
	}
	return 0;
}
