
#include "libft.h"

#include "bootstrap.h"
#include "vga_text.h"

extern void bootstrap_end(void);

extern void payload_get_mem_map(void);
extern size_t payload_get_mem_map_len;

struct __attribute__ ((packed)) device {
	u32 low_addr;
	u32 high_addr;
	u32 low_length;
	u32 high_length;
	u32 type;
	u32 acpi_reserved;
	u32 reserved_a;
	u32 reserved_b;
};

void bootstrap_main(void) {
	clear_screen();
	struct base_registers regs = {0};
	int res = i8086_payload(regs, &payload_get_mem_map, payload_get_mem_map_len);
	if (res == -1) {
		printk("Cannot map devices !\n");
		while (1) {}
	}
	printk("device map detected: %i\n", res);
	struct device *ptr_device = (struct device *)DEVICE_MAP_PTR_ADDR;
	for (int i = 0; i < res; i++) {
		printk("addr: %.8p len %u ko type: %u acpi: %.8x\n",
		       ptr_device->low_addr,
		       ptr_device->low_length >> 10,
		       ptr_device->type,
		       ptr_device->acpi_reserved);
		ptr_device += 1;
	}
	// Mark zero on the last entry
	ft_memset(ptr_device, 0, sizeof(struct device));
	bootstrap_end();
}
