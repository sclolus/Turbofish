#include "internal_printf.h"

static void		f_x(char *s, uintmax_t n, int size)
{
	while (size) {
		s[--size] = HEXTABLE_MIN((n & 0b1111));
		n >>= 4;
	}
}

static void		buffer_p(
			uintmax_t n,
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
	void		*n;
	uintmax_t	i;
	int		params[2];

	n = va_arg(op->ap, void *);
	args->b |= HASH;
	params[0] = (args->p == 0 && n == 0) ? 0 : 1;
	i = (uintmax_t)n;
	while ((i = i >> 4))
		params[0]++;
	params[0] = (args->p > params[0]) ? args->p : params[0];
	params[1] = ((int)args->w > (params[0] + 2)) ? args->w : params[0] + 2;
	buffer_p((uintmax_t)n, args, op, params);
	return (0);
}
