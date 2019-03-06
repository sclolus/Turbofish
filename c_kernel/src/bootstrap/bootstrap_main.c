
#include "libft.h"

#include "bootstrap.h"
#include "vga_text.h"

extern void bootstrap_end(void);

u8 banane[1024] = {0};

extern void payload_get_mem_map(void);
extern size_t payload_get_mem_map_len;

void bootstrap_main(void) {
	ft_memset(0, 0, 0);
	banane[0] = 42;
	clear_screen();
	printk("Les carotes sont cuites\n");
	struct base_registers regs = {0};
	printk("%i\n", i8086_payload(regs, &payload_get_mem_map, payload_get_mem_map_len));
	bootstrap_end();
}
