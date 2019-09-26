#include "internal_printf.h"

static void		f_x(char *s, void *n, int size)
{
	while (size) {
		#if __WORDSIZE == 64
		s[--size] = HEXTABLE_MIN(((long long unsigned)n & 0b1111));
		n = (void *)((long long unsigned)n >> 4);
		#else
		s[--size] = HEXTABLE_MIN(((unsigned)n & 0b1111));
		n = (void *)((unsigned)n >> 4);
		#endif
	}
}

static void		buffer_p(
			void *n,
			t_args *args,
			t_status *op,
			int *params)
{
	int		flag_0;
	int		left_justify;
	int		start_n;
	char		buf[params[1]];

	left_justify = (args->b & MINUS) ? 1 : 0;
	flag_0 = ((args->b & ZERO) && (args->p == -1) && (!(left_justify))) ?
			1 : 0;
	if (left_justify) {
		ft_memset(buf + params[0] + 2, ' ', params[1] - params[0] - 2);
		buf[0] = '0';
		buf[1] = 'x';
		f_x(buf + 2, n, params[0]);
		string_to_buffer(buf, params[1], op);
		return ;
	}
	start_n = params[1] - params[0];
	if (flag_0)
		ft_memset(buf + 2, '0', start_n - 2);
	else
		ft_memset(buf, ' ', start_n - 2);
	buf[(flag_0) ? 0 : start_n - 2] = '0';
	buf[(flag_0) ? 1 : start_n - 1] = 'x';
	f_x(buf + start_n, n, params[0]);
	string_to_buffer(buf, params[1], op);
}

int			s_pointer(t_args *args, t_status *op)
{
	void *n;
	void *i;
	int params[2];

	n = va_arg(op->ap, void *);
	args->b |= HASH;
	params[0] = (args->p == 0 && n == 0) ? 0 : 1;
	i = n;
#if __WORDSIZE == 64
	while ((i = (void *)((long long unsigned)i >> 4)))
#else
	while ((i = (void *)((unsigned)i >> 4)))
#endif
		params[0]++;
	params[0] = (args->p > params[0]) ? args->p : params[0];
	params[1] = ((int)args->w > (params[0] + 2)) ? args->w : params[0] + 2;
	buffer_p(n, args, op, params);
	return (0);
}
