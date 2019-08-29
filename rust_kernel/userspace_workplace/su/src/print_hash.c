#include "su.h"

/* #ifdef TESTS */
void	print_hash(uint32_t *digest, uint64_t size, int32_t swap_endian)
{
	uint32_t	i;
	uint32_t	tmp;

	i = 0;
	while (i < size / 4)
	{
		tmp = digest[i];
		/* if (swap_endian) */
		/* 	tmp = swap_int32(tmp); */
		printf("%8.8x", tmp);
		i++;
	}
}
/* #endif */
