#include "internal_printf.h"

static void		f_i(char *s, intmax_t n, int size)
{
	if (n < 0) {
		while (size--) {
			s[size] = (-(n % 10)) + '0';
			n /= 10;
		}
	} else {
		while (size--) {
			s[size] = (n % 10) + '0';
			n /= 10;
		}
	}
}

static void		buffer_i(
			intmax_t n,
			int *size,
			t_args *args,
			t_status *op)
{
	int		x;
	int		flag_0;
	char		buf[size[1]];

	flag_0 = ((args->b & ZERO) && (!(args->b & MINUS)) && args->p == -1) ?
			1 : 0;
	x = ((n < 0) || (args->b & PLUS) || (args->b & SPACE)) ? 1 : 0;
	if (args->b & MINUS)
		ft_memset(buf + size[0] + x, ' ', size[1] - size[0] - x);
	else
		ft_memset(buf + ((flag_0 && x) ? 1 : 0),
			flag_0 ? '0' : ' ', size[1] - size[0] - x);
	if (n < 0)
		buf[(flag_0 || (args->b & MINUS)) ?
				0 : size[1] - size[0] - 1] = '-';
	else if (args->b & PLUS)
		buf[(flag_0 || (args->b & MINUS)) ?
				0 : size[1] - size[0] - 1] = '+';
	else if (args->b & SPACE)
		buf[(flag_0 || (args->b & MINUS)) ?
				0 : size[1] - size[0] - 1] = ' ';
	f_i(buf + ((args->b & MINUS) ? x : size[1] - size[0]), n, size[0]);
	string_to_buffer(buf, size[1], op);
}

int			s_numeric_i(t_args *args, t_status *op)
{
	intmax_t	n;
	intmax_t	i;
	int		size[2];
	int		x;

	cast_i(&n, args->l, op);
	size[0] = (args->p == 0 && n == 0) ? 0 : 1;
	i = n;
	while ((i = i / 10))
		size[0]++;
	size[0] = (args->p > size[0]) ? args->p : size[0];
	x = ((n < 0) || (args->b & PLUS) || (args->b & SPACE)) ? 1 : 0;
	size[1] = (args->w > (size[0] + x)) ? args->w : size[0] + x;
	buffer_i(n, size, args, op);
	return (0);
}
