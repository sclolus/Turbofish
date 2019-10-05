#include "ifc.h"

int ifc_numeric_d(struct Ctx *ctx, struct Arguments *args)
{
	const char  *decimal_base = "0123456789";
	int	    ret;
	int	    *nbr;

	if (args->to_ignore == false) {
		nbr = (int*)va_arg(ctx->ap, int*);
	} else {
		nbr = NULL;
	}


	trash_whitespaces_on_input(ctx);
	int nb = 0;

	while ((ret = get_content(ctx)) != EOF) {
		char	*in_base;

		if (!(in_base = strchr(decimal_base, (char)ret))) {
			break ;
		}
		size_t	in_base_index = (size_t)(in_base - decimal_base);

		nb *= 10;
		nb += in_base_index;
		consume_content();

		// Fill only if width condition if true
		/* if (args->width == 0 || i < args->width) { */
		/* 	if (args->to_ignore == false) { */
		/* 		nbr[i++] = (char)ret; */
		/* 	} */
		/* 	consume_content(); */
		/* } else { */
		/* 	break; */
		/* } */
	}

	if (args->to_ignore == false) {
		*nbr = nb;
	}

	return 0;
}
