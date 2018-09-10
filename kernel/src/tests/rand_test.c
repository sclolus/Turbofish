
#include "libft.h"
#include "math.h"
#include "vesa_graphic.h"

int	rand_test(void)
{
	srand(0xACE2);

	u32 max_cap = g_graphic_ctx.vesa_mode_info.width / 8 / 28;
	u32 i = 0;

	for (u16 u = 1; u < 65535; u += 1)
	{
		u16 res;
		u32 q = 0;
		u32 j = 0;

		do {
			res = rand(u);
			if (res > u)
			{
				ft_printf("{red}ERROR SUP: %hu > %hu{eoc}\n",
						res, u);
				return -1;
			}
			q += res;
			j++;
		} while (j < ((1 << 10) - 1));
		ft_printf("middle: %.5u for u = %.5hu ", q / j, u);
		i++;
		ft_printf((i % max_cap == 0) ? "\n" : " ");
	}
	if (i % max_cap != 0)
		ft_printf("\n");
	ft_printf("{eoc}");
	ft_printf("RAND TEST DONE\n");
	return 0;
}
