
#include "main_headers.h"

size_t			allign_size(size_t size, enum e_page_type page_type)
{
	if (page_type == TINY) {
		if (size == 0)
			return (32);
		return (((size >> TINY_SHR) +
			((size & TINY_MASK) ? 1 : 0)) << TINY_SHR);
	} else if (page_type == MEDIUM) {
		return (((size >> MEDIUM_SHR) +
			((size & MEDIUM_MASK) ? 1 : 0)) << MEDIUM_SHR);
	}
	return (((size / ctx.page_size) +
		((size % ctx.page_size) ? 1 : 0)) * ctx.page_size);
}

/*
** Simple, basic, just return the page type.
*/

enum e_page_type	get_page_type(size_t size)
{
	if (size <= TINY_LIMIT)
		return (TINY);
	else if (size <= MEDIUM_LIMIT)
		return (MEDIUM);
	else
		return (LARGE);
}
