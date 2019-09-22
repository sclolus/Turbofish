#include "internal_printf.h"

extern int write(int fd, const char *buf, size_t count);

static char g_buf[MAX_BUF_LEN];

#define MIN(X, Y) (((X) < (Y)) ? (X) : (Y))

void	fflush_buffer(t_status *op)
{
	if (op->params == Fd) {
		write(op->opt.fd.fd, g_buf, op->buff_len);
		op->total_size += op->buff_len;
	} else if (op->params == GivenString) {
		op->total_size += (size_t)op->buff_len;
		if (op->opt.given_string.max_size == 0) {
			// Don't do anything if left size == 0
		} else {
			size_t copied_size = MIN((size_t)op->buff_len, op->opt.given_string.max_size);

			ft_memcpy(op->opt.given_string.str, g_buf, copied_size);
			op->opt.given_string.str += copied_size;
			op->opt.given_string.max_size -= copied_size;
		}
	} else {
		if (op->total_size == -1) {
			// In case of previous allocation error, don't do anything
		} else {
			// Keep one byte for '\0'
			char *tmp = realloc(op->opt.given_string.str, op->buff_len + op->total_size + 1);
			if (tmp == NULL) {
				op->total_size = -1;
			} else {
				op->opt.given_string.str = tmp;
				ft_memcpy(op->opt.given_string.str + op->total_size, g_buf, op->buff_len);
				op->total_size += op->buff_len;
			}
		}
	}
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
