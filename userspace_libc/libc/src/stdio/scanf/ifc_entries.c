
#include "libc.h"
#include "ifc.h"

#define STDARG_BOILERPLATE(expr) \
	va_list ap; \
	va_start(ap, format); \
	int n = expr; \
	va_end(ap); \
	return n;

int xscanf(const char *format, ...)
{
	STDARG_BOILERPLATE(xvscanf(format, ap));
}

int xsscanf(const char *str, const char *format, ...)
{
	STDARG_BOILERPLATE(xvsscanf(str, format, ap));
}

int xfscanf(FILE *stream, const char *format, ...)
{
	STDARG_BOILERPLATE(xvfscanf(stream, format, ap));
}

int xvscanf(const char *format, va_list ap)
{
	return xvfscanf(stdin, format, ap);
}

int xvsscanf(const char *str, const char *format, va_list ap)
{
	struct Ctx ctx;

	ctx.flavor = String;
	ctx.input.str = str;
	ctx.ap = ap;

	return parse_chain(&ctx, format);
}

int xvfscanf(FILE *stream, const char *format, va_list ap)
{
	struct Ctx ctx;

	ctx.flavor = Stream;
	ctx.input.stream = stream;
	ctx.ap = ap;

	return parse_chain(&ctx, format);
}
