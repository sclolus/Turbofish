#include "internal_printf.h"

#include <stdio.h>
#include <stdint.h>

#define STDARG_BOILERPLATE(expr) va_list ap; va_start(ap, format); int n = expr; va_end(ap); return n;

/*
 * Derive from stdio.h
 * 'man 3 stdarg' to understand variadics macro.
 */

/*
 * printf, fprintf, dprintf - print to a file descriptor
 */

int printf(const char *format, ...)
{
	STDARG_BOILERPLATE(vprintf(format, ap));
}

int fprintf(FILE *stream, const char *format, ...)
{
	STDARG_BOILERPLATE(vfprintf(stream, format, ap));
}

int dprintf(int const fd, const char *format, ...)
{
	STDARG_BOILERPLATE(vdprintf(fd, format, ap));
}

/*
 * sprintf, snprintf - print to a given string
 */

int sprintf(char *str, const char *format, ...)
{
	STDARG_BOILERPLATE(vsprintf(str, format, ap));
}

int snprintf(char *str, size_t size, const char *format, ...)
{
	STDARG_BOILERPLATE(vsnprintf(str, size, format, ap));
}

/*
 * asprintf - print to allocated string
 */

int asprintf(char **strp, const char *format, ...)
{
	STDARG_BOILERPLATE(vasprintf(strp, format, ap));
}

/*
 * Derive from starg.h
 * 'man 3 stdarg' to understand variadics macro.
 */

/*
 * vprintf, vfprintf, vdprintf - print to a file descriptor
 */

int vprintf(const char* format, va_list ap)
{
	return vdprintf(stdout->fd, format, ap);
}

int vfprintf(FILE *stream, const char *format, va_list ap)
{
	return vdprintf(stream->fd, format, ap);
}

int vdprintf(int fd, const char *format, va_list ap)
{
	t_status	op;
	int		ret;

	ft_memset(&op, 0, sizeof(t_status));
	op.s = format;
	op.fd = fd;
	op.ap = ap;
	ret = new_chain(&op);
	if (ret < 0)
		return (ret);
	fflush_buffer(&op);
	return (op.total_size);
}

/*
 * vsprintf, vsnprintf - print to a given string
 */

int vsprintf(char *str, const char *format, va_list ap)
{
	return vsnprintf(str, SSIZE_MAX, format, ap);
}

int vsnprintf(char *str, size_t size, const char *format, va_list ap)
{
	t_status	op;
	int		ret;

	ft_memset(&op, 0, sizeof(t_status));
	op.s = format;
	op.str = str;
	op.max_size = size;
	op.ap = ap;
	ret = new_chain(&op);
	if (ret < 0)
		return (ret);
	fflush_buffer(&op);
	return (op.total_size);
}

/*
 * vasprintf - print to allocated string
 */

int vasprintf(char **strp, const char *format, va_list ap)
{
	(void)strp;
	(void)format;
	(void)ap;
	return 0;
}

/*
 * Custom implementation
 */

int eprintf(const char *format, ...)
{
	STDARG_BOILERPLATE(vdprintf(stderr->fd, format, ap));
}
