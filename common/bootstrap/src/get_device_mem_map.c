
#include "libft.h"

struct base_registers {
	u32 edi, esi, ebp, esp;
	u32 ebx, edx, ecx, eax;
} __attribute__ ((packed));

/*
 * By default, device memory map will be stored on the 0x40000 address
 */
#define DEVICE_MAP_PTR_ADDR 0x40000

extern int _i8086_payload(struct base_registers *regs, void *payload, size_t payload_len);

extern void payload_get_mem_map(void);
extern size_t payload_get_mem_map_len;

/*
extern int payload_13h_check_extension_present(void);
extern size_t payload_13h_check_extension_present_len;
extern int payload_13h_extended_read_drive_parameters(void);
extern size_t payload_13h_extended_read_drive_parameters_len;
extern int payload_13h_extended_read_sectors(void);
extern size_t payload_13h_extended_read_sectors_len;
extern int payload_13h_extended_write_sectors(void);
extern size_t payload_13h_extended_write_sectors_len;
*/

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
struct __attribute__ ((packed)) drive_parameters {
	u16 result_size;      // size of Result Buffer (set this to 1Eh)
	u16 info_flag;        // information flags
	u32 cylinders;        // physical number of cylinders = last index + 1 (because index starts with 0)
	u32 heads;            // physical number of heads = last index + 1 (because index starts with 0)
	u32 sectors;          // physical number of sectors per track = last index (because index starts with 1)
	u32 nb_sectors_low;   // absolute number of sectors = last index + 1 (because index starts with 0)
	u32 nb_sectors_high;  // absolute number of sectors = last index + 1 (because index starts with 0)
	u16 bytes_per_sector; // bytes per sector
	u32 option_ptr;       // optional pointer to Enhanced Disk Drive (EDD) configuration parameters which may be used for subsequent interrupt 13h Extension calls (if supported)
};

struct __attribute__ ((packed)) dap {
	u8 size_of_dap;       // size of DAP (set this to 10h)
	u8 unused;            // unused, should be zero
	u16 nb_sectors;       // number of sectors to be read, (some Phoenix BIOSes are limited to a maximum of 127 sectors)
	u32 memory;           // segment:offset pointer to the memory buffer to which sectors will be transferred
	u32 sector_low;       // absolute number of the start of the sectors to be read (1st sector of drive has number 0) using logical block addressing.
	u32 sector_high;      // absolute number of the start of the sectors to be read (1st sector of drive has number 0) using logical block addressing.
};

static void check_bios_extended(void) {
	int res;
	struct base_registers regs = {0};

	regs.edx = 0x80;
	res = _i8086_payload(&regs, &payload_13h_check_extension_present, payload_13h_check_extension_present_len);
	if (res == -1) {
		printk("Check extension present failure");
		while (1) {}
	}
	printk("AH: %hx BX: %hx, CX: %hx\n",regs.eax & 0xffff, regs.ebx & 0xffff, regs.ecx & 0xffff);

	struct drive_parameters *params = (struct drive_parameters *)0x80000;
	params->result_size = 0x1E;

	ft_memset(&regs, 0, sizeof(regs));
	regs.edx = 0x80;
	res = _i8086_payload(&regs, &payload_13h_extended_read_drive_parameters, payload_13h_extended_read_drive_parameters_len);
	if (res == -1) {
		printk("Extended read drive parameters failure");
		while (1) {}
	}
	printk("result_size: %hu\n", params->result_size);
	printk("info_flag: %hx\n", params->info_flag);
	printk("cylinders: %u\n", params->cylinders);
	printk("heads: %u\n", params->heads);
	printk("sectors: %u\n", params->sectors);
	printk("nb_sectors_low: %u\n", params->nb_sectors_low);
	printk("nb_sectors_high: %u\n", params->nb_sectors_high);
	printk("bytes_per_sector: %u\n", params->bytes_per_sector);
	printk("option_ptr: %p\n", params->option_ptr);

	struct dap *dap = (struct dap *)0x80000;
	ft_memset(dap, 0, sizeof(*dap));
	dap->size_of_dap = 0x10;
	dap->nb_sectors = 0x1;
	dap->memory = 0x90000000;
	dap->sector_low = 0;
	dap->sector_high = 0;
	res = _i8086_payload(&regs, &payload_13h_extended_read_sectors, payload_13h_extended_read_sectors_len);
	if (res == -1) {
		printk("Extended read sectors failure");
		while (1) {}
	}
	printk("\ndumping result\n");
	for (u8 *i = (u8 *)0x90000; i < (u8 *)0x90200; i++)
		printk("%hhx ", *i);

	ft_memset(dap, 0, sizeof(*dap));
		dap->size_of_dap = 0x10;
	dap->nb_sectors = 0x1;
	dap->memory = 0x90000000;
	dap->sector_low = 0;
	dap->sector_high = 0;
	res = _i8086_payload(&regs, &payload_13h_extended_write_sectors, payload_13h_extended_write_sectors_len);
	if (res == -1) {
		printk("Extended write sectors failure");
		while (1) {}
	}
	printk("\ndumping result\n");
	for (u8 *i = (u8 *)0x90000; i < (u8 *)0x90200; i++)
		printk("%hhx ", *i);
}
*/

/*
 * Return the memory map device
 */
struct device *get_device_mem_map(void) {
	struct base_registers regs = {0};
	int nb_dev = _i8086_payload(&regs, &payload_get_mem_map, payload_get_mem_map_len);
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
//	check_bios_extended();
	return (struct device *)DEVICE_MAP_PTR_ADDR;
}
