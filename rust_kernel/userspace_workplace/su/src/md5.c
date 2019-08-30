#include "su.h"

enum md5_states {
	A = 0,
	B,
	C,
	D,
};

static const uint32_t	g_shift_deltas[64] = {
	7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22,
	5,  9, 14, 20, 5,  9, 14, 20, 5,  9, 14, 20, 5,  9, 14, 20,
	4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23,
	6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
};

static const uint32_t	g_md5_constants[64] = {
	0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee,
	0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
	0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be,
	0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
	0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa,
	0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
	0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed,
	0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
	0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c,
	0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
	0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05,
	0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
	0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039,
	0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
	0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1,
	0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391
};

INLINE static void	md5_padding(void *block_to_pad, uint8_t *clear, uint64_t total_len)
{
	uint64_t	printed_len;

	memset(block_to_pad, 0, 128);
	memcpy(block_to_pad, clear + total_len - total_len % 64, total_len % 64);
	*((uint8_t*)block_to_pad + total_len % 64) |= 0x80;
	printed_len = total_len * 8;
	/* if (!ft_get_endianness()) */
	/* 	printed_len = swap_int64(printed_len); */ // Dunno if I should support that
	((uint64_t*)block_to_pad)[7 + 8 * ((total_len % 64) > 64 - 9)] = printed_len;
	assert(!memcmp(block_to_pad, clear + total_len - total_len % 64,  total_len % 64));
}

# undef F
# undef G
# undef H
# undef I
# define F(B, C, D) ((B) & (C)) | (~(B) & (D))
# define G(B, C, D) ((B) & (D)) | ((C) & ~(D))
# define H(B, C, D) (B) ^ (C) ^ (D)
# define I(B, C, D) (C) ^ (B | ~(D))

INLINE static void	md5_round(uint32_t *block_states, uint32_t *block)
{
	uint32_t	i;
	uint32_t	tmp_f;
	uint32_t	tmp_g;

	i = 0;
 	while (i < 64)
	{
		if (i < 16)
		{
			tmp_f = F(block_states[B], block_states[C], block_states[D]);
			tmp_g = i;
		}
		else if (i < 32)
		{
			tmp_f = G(block_states[B], block_states[C], block_states[D]);
			tmp_g = (i * 5 + 1) % 16;
		}
		else if (i < 48)
		{
			tmp_f = H(block_states[B], block_states[C], block_states[D]);
			tmp_g = (i * 3 + 5) % 16;

		}
		else
		{
			tmp_f = I(block_states[B], block_states[C], block_states[D]);
			tmp_g = (i * 7) % 16;
		}
		tmp_f += block_states[A] + g_md5_constants[i] + block[tmp_g];
		block_states[A] = block_states[D];
		block_states[D] = block_states[C];
		block_states[C] = block_states[B];
		block_states[B] += left_rotate_32(tmp_f, g_shift_deltas[i]);
		i++;
	}
}

# undef F
# undef G
# undef H
# undef I

INLINE static void	md5_main_loop(uint32_t *states
						  , uint8_t *clear
						  , uint8_t *last_block
						  , uint64_t len)
{
	uint64_t	chunk_nbr;
	uint64_t	i;
	uint32_t	block_states[4];

	chunk_nbr = len / 64;
	i = 0;
	while (i < chunk_nbr)
	{
		block_states[A] = states[A];
		block_states[B] = states[B];
		block_states[C] = states[C];
		block_states[D] = states[D];
		md5_round(block_states, (uint32_t*)(void*)(clear + i * 64));
		states[A] += block_states[A];
		states[B] += block_states[B];
		states[C] += block_states[C];
		states[D] += block_states[D];
		i++;
	}

	uint64_t extra_rounds;
	i = 0;
	extra_rounds = 1 + !!((len % 64) > (64 - 9));
	while (i < extra_rounds) {
		block_states[A] = states[A];
		block_states[B] = states[B];
		block_states[C] = states[C];
		block_states[D] = states[D];
		md5_round(block_states, (uint32_t*)(void*)(last_block + i * 64));
		states[A] += block_states[A];
		states[B] += block_states[B];
		states[C] += block_states[C];
		states[D] += block_states[D];
		i++;
	}
}

static void	init_md5(uint32_t *states)
{
	states[A] = 0x67452301;
	states[B] = 0xefcdab89;
	states[C] = 0x98badcfe;
	states[D] = 0x10325476;
}

char	 *md5_hash(char *clear, char *salt)
{
	static uint8_t		last_block[128];
	uint32_t		states[4];
	char			*digest;

	if (clear == NULL || salt == NULL)
		return (NULL);
	uint32_t	clear_len = strlen(clear);
	uint32_t	salt_len = strlen(salt);
	uint32_t	new_len = clear_len + salt_len;

	char		*new = malloc(new_len + 1);

	if (!new) {
		return NULL;
	}
	memcpy(new, clear, clear_len);
	memcpy(new + clear_len, salt, salt_len);
	new[new_len] = 0;
	clear = new;

	md5_padding(last_block, clear, new_len);
	init_md5(states);
	md5_main_loop(states, (uint8_t*)clear, last_block, new_len);
	free(clear);
	if (!(digest = malloc(16 + 1)))
		return (NULL);
	memcpy(digest, states, 16);
	digest[16] = '\0';
	return (digest);
}
