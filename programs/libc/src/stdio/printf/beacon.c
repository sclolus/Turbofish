#include "internal_printf.h"
#include <stdio.h>

/*
**	'man 3 stdarg' to understand variadics macro.
*/

int printf(const char *restrict format, ...)
{
	va_list ap;

	va_start(ap, format);
	int n = vprintf(format, ap);
	va_end(ap);
	return n;
}


int vprintf(const char* format, va_list ap) {
	t_status	op;
	int		ret;

	ft_memset(&op, 0, sizeof(t_status));
	op.s = format;
	op.fd = STDOUT;
	op.ap = ap;
	ret = new_chain(&op);
	va_end(op.ap);
	if (ret < 0)
		return (ret);
	fflush_buffer(&op);
	return (op.total_size);
}

#warning THIS IS BULLSHIT, MUST CALL VFDPRINTF

int vfprintf(FILE *stream, const char *format, va_list ap)
{
	return vprintf(format, ap);
}

#warning THE SIZE PARAM OF THE VSNPRINTF FUNCTION MUST BE CONSIDERED

int vsnprintf(char *str, size_t size, const char *format, va_list ap) {
	t_status	op;
	int		ret;

	(void)size;
	if (str == NULL)
		return (-1);
	ft_memset(&op, 0, sizeof(t_status));
	op.s = format;
	op.str = str;
	op.ap = ap;
	ret = new_chain(&op);
	va_end(op.ap);
	if (ret < 0)
		return (ret);
	fflush_buffer(&op);
	return (op.total_size);
}

#warning THE SIZE PARAM OF THE SNPRINTF FUNCTION MUST BE CONSIDERED

int snprintf(char *str, size_t size, const char *format, ...) {
	va_list ap;

	va_start(ap, format);
	int n = vsnprintf(str, size, format, ap);
	va_end(ap);
	return n;
}

int _dprintf(bool display, const char *restrict format, ...)
{
	t_status	op;
	int		ret;

	if (display == false)
		return (0);
	ft_memset(&op, 0, sizeof(t_status));
	op.s = format;
	op.fd = STDOUT;
	va_start(op.ap, format);
	ret = new_chain(&op);
	va_end(op.ap);
	if (ret < 0)
		return (ret);
	fflush_buffer(&op);
	return (op.total_size);
}

int eprintf(const char *restrict format, ...)
{
	t_status	op;
	int		ret;

	ft_memset(&op, 0, sizeof(t_status));
	op.s = format;
	op.fd = STDERR;
	va_start(op.ap, format);
	ret = new_chain(&op);
	va_end(op.ap);
	if (ret < 0)
		return (ret);
	fflush_buffer(&op);
	return (op.total_size);
}

int fprintf(FILE *stream, const char *format, ...)
{
	return dprintf(stream->fd, format);
}

int dprintf(int const fd, const char *restrict format, ...)
{
	t_status	op;
	int		ret;

	ft_memset(&op, 0, sizeof(t_status));
	op.s = format;
	op.fd = fd;
	va_start(op.ap, format);
	ret = new_chain(&op);
	va_end(op.ap);
	if (ret < 0)
		return (ret);
	fflush_buffer(&op);
	return (op.total_size);
}

int sprintf(char *str, const char *restrict format, ...)
{
	t_status	op;
	int		ret;

	ft_memset(&op, 0, sizeof(t_status));
	op.s = format;
	op.str = str;
	va_start(op.ap, format);
	ret = new_chain(&op);
	va_end(op.ap);
	if (ret < 0)
		return (ret);
	fflush_buffer(&op);
	return (op.total_size);
}
