
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

struct bootstrap_result {
	void *grub_multiboot_structure;
	struct device *device_map;
};

struct bootstrap_result bootstrap_main(void *grub_multiboot_structure) {
	struct bootstrap_result res;

	res.grub_multiboot_structure = grub_multiboot_structure;

	clear_screen();
	struct base_registers regs = {0};
	int nb_dev = i8086_payload(regs, &payload_get_mem_map, payload_get_mem_map_len);
	if (nb_dev == -1) {
		printk("Cannot map devices !\n");
		while (1) {}
	}
	printk("device map detected: %i\n", nb_dev);

	res.device_map = (struct device *)DEVICE_MAP_PTR_ADDR;
	struct device *ptr_device = res.device_map;

	for (int i = 0; i < nb_dev; i++) {
		printk("addr: %.8p len %u ko type: %u acpi: %.8x\n",
		       ptr_device->low_addr,
		       ptr_device->low_length >> 10,
		       ptr_device->type,
		       ptr_device->acpi_reserved);
		ptr_device += 1;
	}
	// Mark zero on the last entry
	ft_memset(ptr_device, 0, sizeof(struct device));
	return res;
}
