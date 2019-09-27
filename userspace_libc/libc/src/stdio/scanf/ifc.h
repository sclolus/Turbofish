#ifndef __IFC_H__
# define __IFC_H__

#include <unistd.h>
#include <stdarg.h>
#include <stdio.h>
#include <stdbool.h>
#include <ctype.h>

enum Flavor {
	Stream,
	String,
};

union Input {
	FILE *stream;
	const char *str;
};

struct Ctx {
	enum Flavor flavor;
	union Input input;
	va_list ap;
};

int parse_chain(struct Ctx *ctx, const char *format);
int convert(struct Ctx *ctx, const char **format);

#endif
