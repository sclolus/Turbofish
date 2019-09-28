#ifndef __IFC_H__
# define __IFC_H__

#include <unistd.h>
#include <stdarg.h>
#include <stdio.h>
#include <stdbool.h>
#include <ctype.h>
#include <assert.h>
#include <stdlib.h>

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

#define SP_LENGTH_VOID        0x00
#define SP_LENGTH_H           0x01
#define SP_LENGTH_L           0x02
#define SP_LENGTH_LEVEL1      0x03 // abstract
#define SP_LENGTH_Z           0x04
#define SP_LENGTH_J           0x08
#define SP_LENGTH_MAJOR       0x80 // abstract
#define SP_LENGTH_HH          0x81
#define SP_LENGTH_LL          0x82
#define SP_LENGTH_LONG_DOUBLE 0xC0

struct Arguments {
	bool to_ignore;
	int width;
	int length;
};

struct ConvertResult {
	struct Arguments args;
	int (*f)(struct Ctx *, struct Arguments *);
};

int ifc_numeric_i(struct Ctx *ctx, struct Arguments *args);
int ifc_numeric_d(struct Ctx *ctx, struct Arguments *args);
int ifc_numeric_u(struct Ctx *ctx, struct Arguments *args);
int ifc_logical_o(struct Ctx *ctx, struct Arguments *args);
int ifc_logical_x(struct Ctx *ctx, struct Arguments *args);
int ifc_float(struct Ctx *ctx, struct Arguments *args);
int ifc_char(struct Ctx *ctx, struct Arguments *args);
int ifc_string(struct Ctx *ctx, struct Arguments *args);
int ifc_pointer(struct Ctx *ctx, struct Arguments *args);

int get_content(struct Ctx *ctx);
void consume_content(void);
void trash_whitespaces_on_input(struct Ctx *ctx);
int parse_chain(struct Ctx *ctx, const char *format);
struct ConvertResult convert(const char **format);

#define IS_SPACE(c) isspace(c) != 0 ? true : false

#endif
