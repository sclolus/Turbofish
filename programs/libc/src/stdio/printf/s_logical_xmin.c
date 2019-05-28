
#include "internal_printf.h"

static void		f_xmin(char *s, uintmax_t n, int size)
{
	while (size) {
		s[--size] = HEXTABLE_MIN((n & 0b1111));
		n >>= 4;
	}
}

static void		buffer_xmin(
			uintmax_t n,
			t_args *args,
			int *params,
			t_status *op)
{
	int		x;
	int		flag_0;
	int		left_justify;
	char		buf[params[1]];

	left_justify = (args->b & MINUS) ? 1 : 0;
	flag_0 = ((args->b & ZERO) && (!(left_justify)) &&
		args->p == -1) ? 1 : 0;
	x = ((n != 0) && (args->b & HASH)) ? 2 : 0;
	if (left_justify)
		ft_memset(
				buf + (params[0] + x),
				' ',
				params[1] - params[0] - x);
	else
		ft_memset(buf + ((flag_0 && x) ? 2 : 0), flag_0 ? '0' : ' ',
			params[1] - params[0] - x);
	if (x) {
		buf[(flag_0 || left_justify) ?
				0 : params[1] - params[0] - 2] = '0';
		buf[(flag_0 || left_justify) ?
				1 : params[1] - params[0] - 1] = 'x';
	}
	f_xmin(
			buf + ((left_justify) ? x : params[1] - params[0]),
			n,
			params[0]);
	string_to_buffer(buf, params[1], op);
}

int			s_logical_xmin(t_args *args, t_status *op)
{
	uintmax_t	n;
	uintmax_t	i;
	int		params[2];
	int		x;

	n = va_arg(op->ap, uintmax_t);
	cast_u(&n, args->l);
	params[0] = (args->p == 0 && n == 0) ? 0 : 1;
	i = n;
	while ((i = i >> 4))
		params[0]++;
	params[0] = (args->p > params[0]) ? args->p : params[0];
	x = ((n != 0) && (args->b & HASH)) ? 2 : 0;
	params[1] = (args->w > (params[0] + x)) ? args->w : params[0] + x;
	buffer_xmin(n, args, params, op);
	return (0);
}
