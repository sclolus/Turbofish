
#include "libc.h"

#include <unistd.h>

#define STDARG_BOILERPLATE(expr) va_list ap; va_start(ap, format); int n = expr; va_end(ap); return n;

enum Flavor {
	FileDescriptor,
	String,
};

union Input {
	const char *str;
	int fd;
};

struct Ctx {
	enum Flavor flavor;
	union Input input;
	va_list ap;
	int length;
};

int xscanf(const char *format, ...)
{
	STDARG_BOILERPLATE(xvscanf(format, ap));
}

int xsscanf(const char *str, const char *format, ...)
{
	STDARG_BOILERPLATE(xvsscanf(str, format, ap));
}

int xvscanf(const char *format, va_list ap)
{
	struct Ctx ctx;

	ctx.flavor = FileDescriptor;
	ctx.input.fd = STDIN_FILENO;
	ctx.ap = ap;
	ctx.length = 0;

	(void)format;
	// int ret = new_chain(&ctx, format);
	// if (ret < 0)
	// 	return (ret);
	
	// fflush_buffer(&ctx);

	return (ctx.length);

}

int xvsscanf(const char *str, const char *format, va_list ap)
{
	struct Ctx ctx;

	ctx.flavor = String;
	ctx.input.str = str;
	ctx.ap = ap;
	ctx.length = 0;

	(void)format;
	// int ret = new_chain(&ctx, format);
	// if (ret < 0)
	// 	return (ret);
	
	// fflush_buffer(&ctx);

	return (ctx.length);
}

