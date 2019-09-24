#include <stdlib.h>
#include <stdint.h>

#warning DUMMY IMPLEMENTATION of bsearch: this is not a bsearch

void *bsearch(const void *key, const void *base, size_t nel,
       size_t width, int (*compar)(const void *, const void *))
{
	if (nel == 0) {
		return NULL;
	}

	size_t	i = 0;

	while (i < nel) {
		const void  *elem = (const void*)((uint8_t*)base + width * i);

		if (0 == compar(key, elem)) {
			return elem;
		}
		i++;
	}

	return NULL;
}
