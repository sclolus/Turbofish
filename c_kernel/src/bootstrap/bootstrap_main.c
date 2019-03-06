
#include "libft.h"

#include "vga_text.h"

extern void bootstrap_end(void);

u8 banane[1024] = {0};

void bootstrap_main(void) {
	ft_memset(0, 0, 0);
	banane[0] = 42;
	clear_screen();
	printk("Les carotes sont cuites\n");
	while (1) {}
	bootstrap_end();
}
