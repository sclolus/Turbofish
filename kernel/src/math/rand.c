
#include "i386_type.h"

#include "math.h"

#define SEQ_SIZE	8192

static u8 g_rand_sequence[SEQ_SIZE];

// Usage of Linear feedback shift register.
// https://wiki.osdev.org/Random_Number_Generator
int	srand(u16 seed)
{
	if (seed == 0)
		return -1;
	u16 lfsr = seed;
	u16 bit;	// Must be 16bit to allow bit<<15 later in the code
	u32 period = 0;

	for (int i = 0; i < SEQ_SIZE; i++)
		g_rand_sequence[i] = 0;

	do {
// taps: 16 14 13 11; feedback polynomial: x^16 + x^14 + x^13 + x^11 + 1
		bit = ((lfsr >> 0) ^ (lfsr >> 2) ^ (lfsr >> 3) ^ (lfsr >> 5))
				& 1;
		lfsr =  (lfsr >> 1) | (bit << 15);
		g_rand_sequence[period >> 3] |= bit << (period & 0x7);
		period++;
	} while (lfsr != seed);

	return 0;
}

u16	rand(u16 cap)
{
	static u32 ptr = 0;

	if (cap == 0)
		return 0;

	u16 bits_size = 0;
	u16 tmp = cap;
	while (tmp) {
		bits_size++;
		tmp >>= 1;
	}
	u16 max_seq_value = (1 << bits_size) - 1;

	u16 seq_value = 0;
	while (bits_size) {
		u16 offset = ptr & 0x7;
		u16 mask = (bits_size < 8) ? (1 << bits_size) - 1 : 0xFF;
		u16 shift_quantity = (bits_size < (8 - offset)) ?
				bits_size : 8 - offset;

		if (seq_value != 0)
			seq_value <<= shift_quantity;

		seq_value |= (g_rand_sequence[ptr >> 3] >> offset) & mask;

		ptr += shift_quantity;
		if (ptr >= (8192 * 8))
			ptr = 0;

		bits_size -= shift_quantity;
	};

	return round((float)cap * ((float)seq_value / max_seq_value));
}
