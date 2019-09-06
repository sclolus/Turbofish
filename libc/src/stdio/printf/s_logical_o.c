#include "internal_printf.h"

static void		f_o(char *s, uintmax_t n, int size)
{
	while (size) {
		s[--size] = (n & 0b111) + '0';
		n >>= 3;
	}
}

static void		buffer_o(
			uintmax_t n,
			t_args *args,
			int *params,
			t_status *op)
{
	int		flag_0;
	int		left_justify;
	int		start_n;
	char		buf[params[1]];

	left_justify = (args->b & MINUS) ? 1 : 0;
	flag_0 = ((args->b & ZERO) && (args->p == -1) && (!(left_justify))) ?
			1 : 0;
	start_n = (left_justify) ? 0 : params[1] - params[0];
	if (left_justify)
		ft_memset(buf + params[0], ' ', params[1] - params[0]);
	else
		ft_memset(buf, (flag_0) ? '0' : ' ', params[1] - params[0] +
			((args->b & HASH) ? 1 : 0));
	if (args->b & HASH) {
		buf[(flag_0) ? 0 : start_n] = '0';
		f_o(buf + start_n + 1, n, params[0] - 1);
	} else {
		f_o(buf + start_n, n, params[0]);
	}
	string_to_buffer(buf, params[1], op);
}

int			s_logical_o(t_args *args, t_status *op)
{
	uintmax_t	n;
	uintmax_t	i;
	int		params[2];

	cast_u(&n, args->l, op);
	params[0] = (!n && args->p == 0) ? 0 : 1;
	i = n;
	while ((i = i >> 3))
		params[0]++;
	if (n) {
		if (args->p <= params[0])
			params[0] += (args->b & HASH) ? 1 : 0;
		else
			args->b &= 0xFD;
	} else {
		params[0] = (args->b & HASH) ? 1 : params[0];
	}
	params[0] = (args->p > params[0]) ? args->p : params[0];
	params[1] = ((int)args->w > params[0]) ? args->w : params[0];
	buffer_o(n, args, params, op);
	return (0);
}
