#include "internal_printf.h"

static int		compare(const char *s1, const char *s2)
{
	int i;

	i = 0;
	while (s2[i] != 0x00 && s1[i] != 0x00 && (s1[i] == s2[i]))
		i++;
	if (s1[i] == 0x00)
		return (i);
	return (0);
}

static const char	*g_modifier_list[MODIFIER_QUANTITY][2] = {
	{ "{eoc}", "\x1B[0m" },
	{ "{red}", "\x1B[31m" },
	{ "{green}", "\x1B[32m" },
	{ "{yellow}", "\x1B[33m" },
	{ "{blue}", "\x1B[34m" },
	{ "{magenta}", "\x1B[35m" },
	{ "{cyan}", "\x1B[36m" },
	{ "{white}", "\x1B[37m" },
	{ "{black}", "\x1B[38m"},
	{ "{orange}", "\x1B[39m"},
	{ "{grey}", "\x1B[40m"},
	{ "{deepblue}", "\x1B[41m"},
	{ "{lightgreen}", "\x1B[42m"}
};

void			assign_modifier(t_status *op)
{
	int l;
	int j;

	l = 0;
	while (l < MODIFIER_QUANTITY) {
		if (compare(g_modifier_list[l][0], op->s)) {
			op->s += strlen(g_modifier_list[l][0]);
			j = strlen(g_modifier_list[l][1]);
			string_to_buffer(g_modifier_list[l][1], j, op);
			return ;
		}
		l++;
	}
	op->s += 1;
	char_to_buffer('{', 1, op);
}
