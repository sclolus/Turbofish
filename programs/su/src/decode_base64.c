#include "su.h"

static INLINE uint32_t	swap_int32(const uint32_t data)
{
	return (((data & 0xff000000) >> 24) |
			((data & 0x00ff0000) >> 8) |
			((data & 0x0000ff00) << 8) |
			((data & 0x000000ff) << 24));
}
// this should be replace with the inverse hash table of the BASE64 char-set
static INLINE uint32_t	decode_32bits(uint8_t byte0, uint8_t byte1
									   , uint8_t byte2, uint8_t byte3)
{
	byte0 *= !(byte0 == '=');
	byte1 *= !(byte1 == '=');
	byte2 *= !(byte2 == '=');
	byte3 *= !(byte3 == '=');
	return (((uint32_t)(strchr(BASE64_CHARS, byte0) - BASE64_CHARS) << 18)
			| ((uint32_t)(strchr(BASE64_CHARS, byte1) - BASE64_CHARS) << 12)
			| ((uint32_t)(strchr(BASE64_CHARS, byte2) - BASE64_CHARS) << 6)
			| (uint32_t)(strchr(BASE64_CHARS, byte3) - BASE64_CHARS));
}

uint8_t					*decode_base64(uint8_t *cipher, uint32_t len)
{
	uint8_t		*clear;
	uint32_t	i;
	uint32_t	clear_index;

	i = 0;
	clear_index = 0;
	if (!(clear = (uint8_t *)malloc(len + 1)))
		return NULL;
	memset(clear, 0, len);
	while (i + 3 < len)
	{
		*((uint32_t*)(void*)(clear + clear_index)) |= swap_int32(decode_32bits(cipher[i], cipher[i + 1], cipher[i + 2], cipher[i + 3]) << 8);
		clear_index += 3;
		i += 4;
	}
	i = len - 4;
	while (i < len)
	{
		if (cipher[i] == '=')
			clear[clear_index - (len - i)] = '\0';
		i++;
	}
	clear[clear_index] = '\0';
	return (clear);
}
