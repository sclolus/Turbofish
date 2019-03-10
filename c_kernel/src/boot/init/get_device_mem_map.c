
#include "libft.h"

struct base_registers {
	u32 edi, esi, ebp, esp;
	u32 ebx, edx, ecx, eax;
} __attribute__ ((packed));

/*
 * By default, device memory map will be stored on the 0x40000 address
 */
#define DEVICE_MAP_PTR_ADDR 0x40000

extern int i8086_payload(struct base_registers regs, void *payload, size_t payload_len);
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

/*
 * Return the memory map device
 */
struct device *get_device_mem_map(void) {
	struct base_registers regs = {0};
	int nb_dev = i8086_payload(regs, &payload_get_mem_map, payload_get_mem_map_len);
	if (nb_dev == -1) {
		printk("Cannot map devices !\n");
		while (1) {}
	}
#ifdef DEV
	printk("device map detected: %i\n", nb_dev);
#endif
	struct device *ptr_device = (struct device *)DEVICE_MAP_PTR_ADDR;

	for (int i = 0; i < nb_dev; i++) {
#ifdef DEV
		printk("addr: %.8p len %u ko type: %u acpi: %.8x\n",
		       ptr_device->low_addr,
		       ptr_device->low_length >> 10,
		       ptr_device->type,
		       ptr_device->acpi_reserved);
#endif
		ptr_device += 1;
	}
	// Mark zero on the last entry
	ft_memset(ptr_device, 0, sizeof(struct device));
	return (struct device *)DEVICE_MAP_PTR_ADDR;
}
