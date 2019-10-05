#include "internal_printf.h"

static t_s_flags g_flags[FLAGS_QUANTITY] = {
	{ '+', PLUS },
	{ ' ', SPACE },
	{ '#', HASH },
	{ '-', MINUS },
	{ '0', ZERO }
};

static t_s_length g_length[LENGTH_TYPE_QUANTITY] = {
	{ 'h', H },
	{ 'l', L },
	{ 'z', Z },
	{ 'j', J }
};

static t_specifier g_sp_list[SPECIFIERS_QUANTITY] = {
	{ 'p', 0xFF, &s_pointer },
	{ 'x', 0x00, &s_logical_xmin },
	{ 'X', 0x00, &s_logical_xmaj },
	{ 'o', 0x00, &s_logical_o },
	{ 'O', 0x02, &s_logical_o },
	{ 'b', 0x00, &s_logical_b },
	{ 'B', 0x02, &s_logical_b },
	{ 'u', 0x00, &s_numeric_u },
	{ 'U', 0x02, &s_numeric_u },
	{ 'i', 0x00, &s_numeric_i },
	{ 'd', 0x00, &s_numeric_i },
	{ 'D', 0x02, &s_numeric_i },
	{ 's', 0x00, &s_string },
	{ 'S', 0x02, &s_string },
	{ 'c', 0x00, &s_char },
	{ 'C', 0x02, &s_char },
	{ 'f', 0x00, &s_float },
};

static int		p_extract_length(const char *restrict s, t_args *args)
{
	const char	*origin;
	int		i;

	origin = s;
	while (true) {
		i = -1;
		while (++i < LENGTH_TYPE_QUANTITY) {
			if (*s == g_length[i].sequence) {
				s++;
				if ((args->l & Z) || (args->l & J))
					break ;
				if (args->l && ((args->l > LEVEL1) ||
					(args->l != g_length[i].value)))
					args->l = 0;
				args->l |= ((args->l & LEVEL1)) ?
						MAJOR : g_length[i].value;
				break ;
			}
		}
		if (i == LENGTH_TYPE_QUANTITY)
			break ;
	}
	return (s - origin);
}

static void		p_extract_wildcard_w(
			char next,
			int *i,
			t_status *op,
			t_args *args)
{
	int x;

	*i += 1;
	x = (int)va_arg(op->ap, int);
	if (next >= '0' && next <= '9') {
		args->w = 0;
		return ;
	}
	if ((args->w = x) < 0) {
		args->w = -1 * args->w;
		args->b |= MINUS;
	}
}

static void		p_extract_wildcard_p(int *i, t_status *op, t_args *args)
{
	*i += 1;
	if ((args->p = (int)va_arg(op->ap, int)) < 0)
		args->p = -1;
}

static void		p_extract_all_stuff(
			const char *restrict s,
			t_args *args,
			int *i,
			t_status *op)
{
	int j;

	if (s[*i] == '*')
		p_extract_wildcard_w(s[*i + 1], i, op, args);
	if (s[*i] >= '0' && s[*i] <= '9' && args->w == 0) {
		while (s[*i] >= '0' && s[*i] <= '9')
			args->w = args->w * 10 + (s[(*i)++] - '0');
	}
	if (s[*i] == '.') {
		*i += 1;
		if (s[*i] == '*') {
			p_extract_wildcard_p(i, op, args);
		} else {
			args->p = 0;
			while (s[*i] >= '0' && s[*i] <= '9')
				args->p = args->p * 10 + (s[(*i)++] - '0');
		}
	}
	if ((j = p_extract_length((s + *i), args)))
		*i += j;
}

void			get_args(
			const char *restrict s,
			int *i,
			t_args *args,
			t_status *op)
{
	int j;
	int base;

	base = -1;
	while (*i != base && s[*i]) {
		base = *i;
		j = -1;
		while (++j < FLAGS_QUANTITY) {
			if (s[*i] == g_flags[j].flag) {
				args->b |= g_flags[j].value;
				(*i)++;
			}
		}
		p_extract_all_stuff(s, args, i, op);
		j = -1;
		while (++j < SPECIFIERS_QUANTITY) {
			if (s[(*i)] == g_sp_list[j].specifier) {
				(*i)++;
				args->f = g_sp_list[j].f;
				args->l = (g_sp_list[j].sp_len) ?
						g_sp_list[j].sp_len : args->l;
				return ;
			}
		}
	}
}
