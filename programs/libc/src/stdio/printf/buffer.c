#include "internal_printf.h"

extern int write(int fd, const char *buf, size_t count);

static char g_buf[MAX_BUF_LEN];

void	fflush_buffer(t_status *op)
{
	if (!op->str) {
		write(op->fd, g_buf, op->buff_len);
	} else {
		ft_memcpy(op->str, g_buf, op->buff_len);
		op->str += op->buff_len;
	}
	op->total_size += op->buff_len;
	op->buff_len = 0;
}

void	string_to_buffer(const char *s, int len, t_status *op)
{
	int i;

	while (len > (MAX_BUF_LEN - op->buff_len)) {
		i = MAX_BUF_LEN - op->buff_len;
		ft_memcpy(g_buf + op->buff_len, s, i);
		s += i;
		op->buff_len += i;
		fflush_buffer(op);
		len -= i;
	}
	ft_memcpy(g_buf + op->buff_len, s, len);
	op->buff_len += len;
}

void	char_to_buffer(char c, int len, t_status *op)
{
	int i;

	while (len > (MAX_BUF_LEN - op->buff_len)) {
		i = MAX_BUF_LEN - op->buff_len;
		ft_memset(g_buf + op->buff_len, c, i);
		op->buff_len += i;
		fflush_buffer(op);
		len -= i;
	}
	ft_memset(g_buf + op->buff_len, c, len);
	op->buff_len += len;
}
