
#include "libft.h"
#include "math.h"
#include "vesa.h"

int	rand_test(void)
{
	srand(0xACE2);

	u32 max_cap = vesa_ctx.mode.width / 8 / 28;
	u32 i = 0;

	for (u16 u = 1; u < 65535; u += 1) {
		u16 res;
		u32 q = 0;
		u32 j = 0;

		do {
			res = rand(u);
			if (res > u) {
				printk("{red}ERROR SUP: %hu > %hu{eoc}\n",
						res, u);
				return -1;
			}
			q += res;
			j++;
		} while (j < ((1 << 10) - 1));
		printk("middle: %.5u for u = %.5hu ", q / j, u);
		i++;
		printk((i % max_cap == 0) ? "\n" : " ");
	}
	if (i % max_cap != 0)
		printk("\n");
	printk("{eoc}");
	printk("RAND TEST DONE\n");
	return 0;
}
