#include "su.h"

/* #ifdef TESTS */
void	print_hash(const char *const digest)
{
	uint32_t	i;
	size_t		len = strlen(digest);

	i = 0;
	while (i < len)
	{
		printf("%2.2hhx", digest[i]);
		i++;
	}
}
/* #endif */
