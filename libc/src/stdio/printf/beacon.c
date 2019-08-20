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
	op.ap = ap;

	op.opt.fd.fd = fd;
	op.params = Fd;

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

	if (size == 0) {
		return 0;
	} else if (size == 1) {
		str[0] = '\0';
		return 1;
	}

	ft_memset(&op, 0, sizeof(t_status));
	op.s = format;
	op.ap = ap;

	op.opt.given_string.str = str;
	// Keep one byte to write the '\0'
	op.opt.given_string.max_size = size - 1;
	op.params = GivenString;

	ret = new_chain(&op);
	if (ret < 0)
		return (ret);
	fflush_buffer(&op);
	// Write the terminated byte '\0' and return copied size + 1
	*op.opt.given_string.str = '\0';
	return (op.total_size + 1);
}

/*
 * vasprintf - print to allocated string
 */

int vasprintf(char **strp, const char *format, va_list ap)
{
	t_status	op;
	int		ret;

	ft_memset(&op, 0, sizeof(t_status));
	op.s = format;
	op.ap = ap;

	// Preallocate one byte for the '\0'
	op.opt.allocated_string.str = malloc(1);
	if (op.opt.allocated_string.str == NULL) {
		return -1;
	}
	op.params = AllocatedString;

	ret = new_chain(&op);
	if (ret < 0) {
		if (op.opt.allocated_string.str != NULL)
			free(op.opt.allocated_string.str);
		return (ret);
	}
	fflush_buffer(&op);
	if (op.total_size == -1) {
		if (op.opt.allocated_string.str != NULL)
			free(op.opt.allocated_string.str);
		return -1;
	}

	// Write the terminated byte '\0' and return copied size + 1
	op.opt.allocated_string.str[op.total_size] = '\0';
	*strp = op.opt.allocated_string.str;
	return (op.total_size + 1);
}

/*
 * Custom implementation
 */

int eprintf(const char *format, ...)
{
	STDARG_BOILERPLATE(vdprintf(stderr->fd, format, ap));
}
