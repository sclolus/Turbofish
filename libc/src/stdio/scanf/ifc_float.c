#include "ifc.h"

union real {
	float *f;
	double *d;
	long double *ld;
};

#define BUF_MAX_SIZE 512

int get_sign(struct Ctx *ctx, struct Arguments *args, char *buf, int i)
{
	int ret;

	while ((args->width == 0 || i < args->width)
			&& (ret = get_content(ctx)) != EOF
			&& ((char)ret == '+' || (char)ret == '-')) {
		buf[i++] = (char)ret;
		if (i == BUF_MAX_SIZE) {
			return -1;
		}
		consume_content();
	}
	return i;
}

int get_digits(struct Ctx *ctx, struct Arguments *args, char *buf, int i)
{
	int ret;

	while ((args->width == 0 || i < args->width)
			&& (ret = get_content(ctx)) != EOF
			&& (char)ret >= '0' && (char)ret <= '9') {
		buf[i++] = (char)ret;
		if (i == BUF_MAX_SIZE) {
			return -1;
		}
		consume_content();
	}
	return i;
}

static bool multiple_chr(char ref, const char *charset) {
	size_t len = strlen(charset);

	for (size_t i = 0; i < len; i++) {
		if (charset[i] == ref) {
			return true;
		}
	}
	return false;
}

int get_raw_symbols(
		struct Ctx *ctx,
		struct Arguments *args,
		char *buf,
		int i,
		const char *charset)
{
	int ret;

	while ((args->width == 0 || i < args->width)
			&& (ret = get_content(ctx)) != EOF
			&& multiple_chr((char)ret, charset) == true) {
		buf[i++] = (char)ret;
		if (i == BUF_MAX_SIZE) {
			return -1;
		}
		consume_content();
	}
	return i;
}
/*
 * A series of decimal digits, optionally containing a decimal point, optionally
 * preceeded by a sign (+ or -) and optionally followed by the e or E character
 * and a decimal integer (or some of the other sequences supported by strtod).
 * Implementations complying with C99 also support hexadecimal floating-point
 * format when preceded by 0x or 0X.
 */
int ifc_float(struct Ctx *ctx, struct Arguments *args)
{
	char buf[BUF_MAX_SIZE];
	union real real;
	int i;

	if (args->to_ignore == false) {
		if (args->length == SP_LENGTH_L) {
			real.d = va_arg(ctx->ap, double *);
		} else if (args->length == SP_LENGTH_LONG_DOUBLE) {
			real.ld = va_arg(ctx->ap, long double *);
		} else {
			real.f = va_arg(ctx->ap, float *);
		}
	}
	trash_whitespaces_on_input(ctx);
	i = 0;

	// Consider the sign
	if ((i = get_sign(ctx, args, buf, i)) < 0) {
		return -1;
	}
	// Run away on the mantissa
	if ((i = get_digits(ctx, args, buf, i)) < 0) {
		return -1;
	}
	// Consider the floating point
	if ((i = get_raw_symbols(ctx, args, buf, i, ".")) < 0) {
		return -1;
	}

	// Run away after the floating point
	if ((i = get_digits(ctx, args, buf, i)) < 0) {
		return -1;
	}

	// No real parts was found: Return an error
	if (i == 0) {
		return -1;
	}

	// Consider the exponent
	if ((i = get_raw_symbols(ctx, args, buf, i, "eE")) < 0) {
		return -1;
	}

	// Consider the sign
	if ((i = get_sign(ctx, args, buf, i)) < 0) {
		return -1;
	}
	// Run away after the exponent
	if ((i = get_digits(ctx, args, buf, i)) < 0) {
		return -1;
	}

	if (i == BUF_MAX_SIZE) {
		return -1;
	}
	// Terminate the buff
	buf[i] = '\0';

	// Finally, fill the real
	if (args->to_ignore == false) {
		if (args->length == SP_LENGTH_L) {
			*real.d = strtod(buf, NULL);
		} else if (args->length == SP_LENGTH_LONG_DOUBLE) {
			/* *real.ld = strtold(buf, NULL); */
			*real.ld = (long double)strtod(buf, NULL);
		} else {
			/* *real.f = strtof(buf, NULL); */
			*real.f = (float)strtod(buf, NULL);
		}
	}
	return 0;
}
