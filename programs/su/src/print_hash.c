#include "su.h"

void	print_hash(const char *const hash)
{
	uint32_t	i;
	uint8_t		*digest = (uint8_t*)hash;

	i = 0;
	while (i < 16)
	{
		printf("%02x", digest[i]);
		i++;
	}
}
