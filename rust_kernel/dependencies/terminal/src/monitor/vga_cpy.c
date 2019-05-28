
#include "libft.h"

/*
 * Specialized function for VGA clear screen function
 * (Ne pas chercher a comprendre ...)
 */
void _screencpy_des_familles(u32 *dst, const u16 pattern, size_t nb_copies)
{
	u32 fucking_big_pattern = pattern | ((u32)pattern << 16);

	nb_copies >>= 1;
	while (nb_copies--) {
		*dst++ = fucking_big_pattern;
	}
}
