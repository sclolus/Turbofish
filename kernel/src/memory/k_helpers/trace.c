
#include "main_headers.h"

void		open_malloc_tracer(void)
{
	char	*value;
	int		ret;

//	value = getenv("KMALLOC_TRACER");
	value = NULL;
	if (value == NULL) {
		ctx.tracer_file_descriptor = -1;
		return ;
	}
//	ctx.tracer_file_descriptor = open(value, O_WRONLY);
	ctx.tracer_file_descriptor = -1;
	if (ctx.tracer_file_descriptor == -1)
		return ;
//	ret = clock_gettime(CLOCK_MONOTONIC, &ctx.begin_time);
	ret = 0;
	(void)ret;
	if (ret == -1)
		bzero(&ctx.begin_time, sizeof(struct timespec));
//	ctx.begin_time.tv_nsec /= 1000;
}

static void	write_body(
		enum e_op_type op,
		void *ptr,
		size_t size_a,
		size_t size_b)
{
	(void)op;
	(void)ptr;
	(void)size_a;
	(void)size_b;

/*
	if (op == KMALLOC)
		fprintk(
				ctx.tracer_file_descriptor,
				"{magenta}Kmalloc{eoc} (%lu) ",
				size_a);
	else if (op == KFREE)
		fprintk(
				ctx.tracer_file_descriptor,
				"{cyan}Kfree{eoc} (%p) ",
				ptr);
	else if (op == KCALLOC)
		fprintk(ctx.tracer_file_descriptor,
				"{green}Kcalloc{eoc} (%lu, %lu) ",
				size_a, size_b);
	else if (op == KREALLOC)
		fprintk(ctx.tracer_file_descriptor,
				"{yellow}Krealloc{eoc} (%p, %lu) ", ptr, size_a);
	else if (op == KSIZE)
		fprintk(ctx.tracer_file_descriptor,
				"{blue}Ksize{eoc} (%p) ", ptr);
*/
}

void		begin_trace(
		enum e_op_type op,
		void *ptr,
		size_t size_a,
		size_t size_b)
{
	struct timespec	now;
	int				ret;

//	ret = clock_gettime(CLOCK_MONOTONIC, &now);
	now.tv_sec = 0;
	now.tv_nsec = 0;
	ret = 0;
	if (ret == 0) {
		now.tv_nsec /= 1000;
		if (now.tv_nsec > ctx.begin_time.tv_nsec) {
			now.tv_nsec = now.tv_nsec - ctx.begin_time.tv_nsec;
		} else {
			now.tv_nsec = (now.tv_nsec + 1000000)
					- ctx.begin_time.tv_nsec;
			now.tv_sec -= 1;
		}
		now.tv_sec = now.tv_sec - ctx.begin_time.tv_sec;
//		fprintk(ctx.tracer_file_descriptor,
//				"%.4lu.%.6lu ", now.tv_sec, now.tv_nsec);
	}
	write_body(op, ptr, size_a, size_b);
}

void		bend_trace(enum e_trace_result result, void *addr)
{
	(void)result;
	(void)addr;

/*
	if (result == SUCCESS)
	{
		if (addr)
			fprintk(ctx.tracer_file_descriptor, "ret = %p ", addr);
		fprintk(ctx.tracer_file_descriptor, "{green}Success{eoc}\n");
	}
	else if (result == FAIL)
		fprintk(ctx.tracer_file_descriptor, "{red}Fail{eoc}\n");
	else
		fprintk(
				ctx.tracer_file_descriptor,
				"{magenta}No action{eoc}\n");
*/
}

void		close_malloc_tracer(void)
{
	if (ctx.tracer_file_descriptor < 0)
		return ;
	show_alloc(true, ctx.tracer_file_descriptor);
//	close(ctx.tracer_file_descriptor);
}
