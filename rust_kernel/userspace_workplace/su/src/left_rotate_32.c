#include "su.h"

/*
** Result is undefined if delta >= 32.
*/

INLINE uint32_t  left_rotate_32(uint32_t word, uint32_t delta)
{
	return ((word << delta) | (word >> ((sizeof(int32_t) * 8) - delta)));
}
