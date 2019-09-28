#include "ifc.h"

#define SPECIFIERS_QUANTITY 12

struct SpecifierFunctionRow {
	char specifier;
	int (*f)(struct Ctx *, struct Arguments *);
};

static struct SpecifierFunctionRow g_sp_list[SPECIFIERS_QUANTITY] = {
	{ 'i', &ifc_numeric_i },
	{ 'd', &ifc_numeric_d },
	{ 'u', &ifc_numeric_u },
	{ 'o', &ifc_logical_o },
	{ 'x', &ifc_logical_x },
	{ 'f', &ifc_float },
	{ 'e', &ifc_float },
	{ 'g', &ifc_float },
	{ 'a', &ifc_float },
	{ 'c', &ifc_char },
	{ 's', &ifc_string },
	{ 'p', &ifc_pointer },
};

#define LENGTH_TYPE_QUANTITY 5

struct SpecifierLengthRow {
	char sequence;
	int value;
};

static struct SpecifierLengthRow g_length[LENGTH_TYPE_QUANTITY] = {
	{ 'h', SP_LENGTH_H },
	{ 'l', SP_LENGTH_L },
	{ 'z', SP_LENGTH_Z },
	{ 'j', SP_LENGTH_J },
	{ 'L', SP_LENGTH_LONG_DOUBLE},
};

/*
 * %[*][width][length]specifier
 */

int convert(struct Ctx *ctx, const char **format)
{
	struct Arguments args;

	assert(**format == '%');
	*format += 1;
	// extract star
	if (**format == '*') {
		args.to_ignore = true;
		*format += 1;
	} else {
		args.to_ignore = false;
	}
	// extract width
	args.width = 0;
	while (**format != '\0' && **format >= '0' && **format <= '9') {
		args.width = args.width * 10 + (**format - '0');
		*format += 1;
	}

	// extract length
	args.length = SP_LENGTH_VOID;
	int i;
length_loop:
	for (i = 0; i < LENGTH_TYPE_QUANTITY; i++) {
		if (**format == g_length[i].sequence) {
			*format += 1;
			if ((args.length & SP_LENGTH_Z) ||
					(args.length & SP_LENGTH_J))
				break;
			if (args.length && ((args.length > SP_LENGTH_LEVEL1) ||
					(args.length != g_length[i].value))) {
				args.length = 0;
			}
			args.length |= ((args.length & SP_LENGTH_LEVEL1)) ?
					SP_LENGTH_MAJOR : g_length[i].value;
			break;
		}
	}
	if (i < LENGTH_TYPE_QUANTITY) {
		goto length_loop;
	}

	// Finally call the associated function
	for (i = 0; i < SPECIFIERS_QUANTITY; i++) {
		if (**format == g_sp_list[i].specifier) {
			*format += 1;
			return g_sp_list[i].f(ctx, &args);
		}
	}
	return -1;
}
