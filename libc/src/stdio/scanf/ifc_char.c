#include "ifc.h"

int ifc_char(struct Ctx *ctx, struct Arguments *args)
{
	int ret;
	char *str;
	int i;

	if (args->to_ignore == false) {
		str = (char *)va_arg(ctx->ap, char *);
	} else {
		str = NULL;
	}

	// this should not be systematically trash during %c conversion
	trash_whitespaces_on_input(ctx);
	i = 0;

	if (args->width == 0) {
		args->width = 1;
	}

	while ((ret = get_content(ctx)) != EOF && isgraph((char)ret)) {
		// Fill only if width condition if true
		if (args->width == 0 || i < args->width) {
			if (args->to_ignore == false) {
				str[i++] = (char)ret;
			}
			consume_content();
		} else {
			break;
		}
	}
	if (args->to_ignore == false) {
		str[i] = '\0';
	}
	return 0;
}
