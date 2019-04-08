
#include "libft.h"

/*
 * https://wiki.osdev.org/RSDP
 */

// ACPI version 1 RSDP structure
struct rsdp_descriptor {
	char signature[8];
	u8 checksum;
	char oem_id[6];
	u8 revision;
	u32 rsdt_address;
} __attribute__ ((packed));

// ACPI version 2+ RSDP structure
struct rsdp_descriptor_2 {
	struct rsdp_descriptor first_part;

	u32 length;
	u32 xsdt_address_0_31;
	u32 xsdt_address_32_63;
	u8 extended_checksum;
	u8 reserved[3];
} __attribute__ ((packed));

// ACPI version 1 RSDT structure
struct acpi_rsdt_header {
	char signature[4];
	u32 length;
	u8 revision;
	u8 checksum;
	char oem_id[6];
	char oem_table_id[8];
	u32 oem_revision;
	u32 creator_id;
	u32 creator_revision;
} __attribute__ ((packed));

// ACPI version 2+ XSDT structure
struct acpi_xsdt_header {
	char signature[4];
	u32 length;
	u8 revision;
	u8 checksum;
	char oem_id[6];
	char oem_table_id[8];
	u32 oem_revision;
	u32 creator_id;
	u32 creator_revision;
} __attribute__ ((packed));

/*
 * https://wiki.osdev.org/RSDT
 */
struct rsdt {
	struct acpi_rsdt_header h;
	struct rsdt *others_rsdt;
} __attribute__ ((packed));

/*
 * https://wiki.osdev.org/XSDT
 */
struct xsdt {
	struct acpi_xsdt_header h;
	// On 64b mode, pointer is on 8 bytes
	struct xsdt *others_rsdt_0_31;
	u32 others_rsdt_32_63;
} __attribute__ ((packed));

/*
 * https://wiki.osdev.org/fadt
 */

struct generic_address_structure {
	u8 address_space;
	u8 bit_width;
	u8 bit_offset;
	u8 access_size;
	u32 address_0_31;
	u32 address_32_63;
} __attribute__ ((packed));

struct fadt {
	struct acpi_rsdt_header h; // len = 36
	u32 firmware_ctrl;
	u32 dsdt;

	// field used in ACPI 1.0; no longer in use, for compatibility only
	u8 reserved;

	u8 preferred_power_management_profile;
	u16 sci_interrupt;
	u32 smi_command_port;
	u8 acpi_enable;
	u8 acpi_disable;
	u8 s4_bios_req;
	u8 p_state_control;
	u32 pm1_a_event_block;
	u32 pm1_b_event_block;
	u32 pm1_a_control_block; // -> PM1a_cnt (command port 1)
	u32 pm1_b_control_block; // -> PM1b_cnt (command port 2)
	u32 pm2_control_block;
	u32 pm_timer_block;
	u32 gpe0_block;
	u32 gpe1_block;
	u8 pm1_event_length;
	u8 pm1_control_length;
	u8 pm2_control_length;
	u8 pm_timer_length;
	u8 gpe0_length;
	u8 gpe1_length;
	u8 gpe1_base;
	u8 c_state_control;
	u16 worst_c2_latency;
	u16 worst_c3_latency;
	u16 flush_size;
	u16 flush_stride;
	u8 duty_offset;
	u8 duty_width;
	u8 day_alarm;
	u8 month_alarm;
	u8 century;

	// reserved in ACPI 1.0; used since ACPI 2.0+
	u16 boot_architecture_flags;

	u8 reserved_2;
	u32 flags;

	// 12 byte structure; see below for details
	struct generic_address_structure reset_reg;

	u8 reset_value;
	u8 reserved_3[3];

	// 64bit pointers - Available on ACPI 2.0+
	u32 x_firmware_control_0_31;
	u32 x_firmware_control_32_63;
	u32 x_dsdt_0_31;
	u32 x_dsdt_32_63;

	struct generic_address_structure x_pm1_a_event_block;
	struct generic_address_structure x_pm1_b_event_block;
	struct generic_address_structure x_pm1_a_control_block;
	struct generic_address_structure x_pm1_b_control_block;
	struct generic_address_structure x_pm2_control_block;
	struct generic_address_structure x_pm_timer_block;
	struct generic_address_structure x_gpe0_block;
	struct generic_address_structure x_gpe1_block;
} __attribute__ ((packed));

/*
* Intel ACPI Component Architecture
* AML Disassembler version 20090123
*
* Disassemble of dsdt.aml, Mon May  6 20:41:40 2013
*
*
* Original Table Header:
*     Signature        "DSDT"
*     Length           0x00003794 (14228)
*     Revision         0x01 **** ACPI 1.0, no 64-bit math support
*     Checksum         0x46
*     OEM ID           "DELL"
*     OEM Table ID     "dt_ex"
*     OEM Revision     0x00001000 (4096)
*     Compiler ID      "INTL"
*     Compiler Version 0x20050624 (537200164)
*/

struct dsdt_header {
	/*0 */  char signature[4];
	/*4 */  u32 length;
	/*8 */  u8 revision;
	/*9 */  u8 checksum;
	/*10 */ char oemid[4];
	/*16 */ char oem_table_id[8];
	/*24 */ u32 oem_revision;
	/*28 */ char compiler_id[4];
	/*32 */ u32 compiler_version;
} __attribute__ ((packed));

//
// bytecode of the \_S5 object
// -----------------------------------------
//        | (optional) |    |    |    |
// NameOP | \          | _  | S  | 5  | _
// 08     | 5A         | 5F | 53 | 35 | 5F
//
// -----------------------------------------------------------------------------------------------------------
//           |           |              | ( SLP_TYPa   ) | ( SLP_TYPb   ) | ( Reserved   ) | (Reserved    )
// PackageOP | PkgLength | NumElements  | byteprefix Num | byteprefix Num | byteprefix Num | byteprefix Num
// 12        | 0A        | 04           | 0A         05  | 0A          05 | 0A         05  | 0A         05
//
//----this-structure-was-also-seen----------------------
// PackageOP | PkgLength | NumElements |
// 12        | 06        | 04          | 00 00 00 00
//
// (Pkglength bit 6-7 encode additional PkgLength bytes [shouldn't be the case here])
//
// PackageOP

struct s5_object {
	u8 package_op;
	u8 pkg_length;
	u8 num_elements;
	u8 slp_typ_a_byteprefix;
	u8 slp_typ_a_num;
	u8 slp_typ_b_byteprefix;
	u8 slp_typ_b_num;
} __attribute__ ((packed));

// search a sized pattern in a designed memory area
u8 *memschr(u8 *base_mem, size_t range, u8 *expr, size_t len_expr, bool (*contraint)(void *))
{
	if (len_expr == 0)
		return (u8 *)-1;
	// Be careful with overflow
	if ((size_t)base_mem > U32_MAX - range)
		return (u8 *)-1;
	u8 *end_mem = (u8 *)((size_t)base_mem + range);
	while ((size_t)base_mem + len_expr < (size_t)end_mem) {
		size_t len = 0;
		while (base_mem[len] == expr[len] && len < len_expr)
			len++;
		if (len == len_expr) {
			if (contraint == NULL || contraint((void *)base_mem) == true)
				return base_mem;
		}
		base_mem++;
	}
	return (u8 *)-1;
}

// checksum for rsdp descriptor
bool rsdp_checksum(void *rsdp_descriptor)
{
	size_t len = sizeof(struct rsdp_descriptor);
	u32 checksum = 0;
	while (len--)
		checksum += ((u8 *)rsdp_descriptor)[len];
	if ((checksum & 0xff) == 0)
		return true;
	else
		return false;
}

// checksum for rsdt descriptor
bool rsdt_checksum(void *rsdt_descriptor)
{
	struct acpi_rsdt_header *acpi_rsdt_header = (struct acpi_rsdt_header *)rsdt_descriptor;

	size_t len = (size_t)acpi_rsdt_header->length;
	u32 checksum = 0;
	while (len--)
		checksum += ((u8 *)acpi_rsdt_header)[len];
	if ((checksum & 0xff) == 0)
		return true;
	else
		return false;
}

// look for FADT acpi descriptor field
struct fadt *find_fscp(void *root_sdt)
{
	struct rsdt *rsdt = (struct rsdt *)root_sdt;
	int entries = (rsdt->h.length - sizeof(rsdt->h)) / 4;

	for (int i = 0; i < entries; i++) {
		struct acpi_rsdt_header *h = &((struct acpi_rsdt_header *)rsdt->others_rsdt)[i];
		if (!strncmp(h->signature, "FACP", 4)) {
			printk("iteration %i / %i: sign = ", i, entries);
			for (int i = 0; i < 4; i++)
				printk("%c", h->signature[i]);
			printk("\n");
			return (struct fadt *)h;
		}
	}
	// No FACP found
	return NULL;
}

inline void outb(u16 port, u8 value)
{
	asm volatile("outb %%al,%%dx"::"d" (port), "a" (value));
}

inline void outw(u16 port, u16 value)
{
	asm volatile("outw %%ax,%%dx"::"d" (port), "a" (value));
}

inline u8 inb(u16 port)
{
	u8 ret;

	asm volatile("inb %%dx,%%al":"=a" (ret): "d" (port));
	return ret;
}

inline u16 inw(u16 port)
{
	u16 ret;

	asm volatile("inw %%dx,%%ax":"=a" (ret): "d" (port));
	return ret;
}

int acpi_enable(struct fadt *fadt) {
	printk("acpi initial state: %hx\n", inw((u16)(fadt->pm1_a_control_block)));

	while ((inw((u16)(fadt->pm1_a_control_block)) & 0x1) != 1) {
		asm("nop");
		outb((u16)(fadt->smi_command_port), fadt->acpi_enable);
	}
	for (int i = 0; i < 2000000000; i++)
		asm("nop");

	printk("acpi final state: %hx\n", inw((u16)(fadt->pm1_a_control_block)));

	for (int i = 0; i < 2000000000; i++)
		asm("nop");

	return 0;
}

const u16 SLP_EN = (1 << 13);

int process_facp(struct acpi_rsdt_header *acpi_rsdt_header)
{
	struct fadt *fadt = find_fscp((void *)acpi_rsdt_header);
	if (fadt != NULL) {

		acpi_enable(fadt);

		struct dsdt_header *dsdt_header = (struct dsdt_header *)fadt->dsdt;

		printk("dsdt_header: signature=%c%c%c%c revision=%u ...\n",
				dsdt_header->signature[0], dsdt_header->signature[1],
				dsdt_header->signature[2], dsdt_header->signature[3],
				dsdt_header->revision);

		u8 *dsdt = (u8 *)dsdt_header;
		u8 *pdsdt = memschr(dsdt, dsdt_header->length, (u8 *)"_S5_", 4, NULL);

		struct s5_object *s5_obj = (struct s5_object *)(pdsdt + 4);

		printk("package_op: %hhx, pkg_length: %hhx, num_elements: %hhx\n",
				s5_obj->package_op, s5_obj->pkg_length, s5_obj->num_elements);
		printk("A => byteprefix: %hhx, key: %hhx\n",
				s5_obj->slp_typ_a_byteprefix, s5_obj->slp_typ_a_num);
		printk("B => byteprefix: %hhx, key: %hhx\n",
				s5_obj->slp_typ_b_byteprefix, s5_obj->slp_typ_b_num);
		printk("A control block: %x\n", fadt->pm1_a_control_block);
		printk("B control block: %x\n", fadt->pm1_b_control_block);

		if (s5_obj->package_op == 0x12) {
			asm volatile("cli");
			printk("preparing shutdown:\n");
			outw((u16)(fadt->pm1_a_control_block), (u16)(((u16)(s5_obj->slp_typ_a_num) << 10) | SLP_EN));
			if (fadt->pm1_b_control_block != 0) {
				outw((u16)(fadt->pm1_b_control_block), (u16)(((u16)(s5_obj->slp_typ_b_num) << 10) | SLP_EN));
			}
			asm volatile("hlt");
			return 0;
		} else {
			printk("no package descriptor was found !\n");
		}
	}
	printk("FADT structure not founded ! Critical ACPI error\n");
	return -1;
	// Look for 8042 ps/2 chipset
	// printk("founded state: %p\n", fadt);
	// printk("flags %0#.4hx for IA boot_architecture_flags\n", fadt->boot_architecture_flags);
	// if ((fadt->boot_architecture_flags & 0x2) != 0)
	//    printk("8042 founded !");
	// else
	//    printk("8042 not founded !");
	// return 0;
}

struct acpi_rsdt_header	*rsdt_stage(void *sdt_descriptor)
{
	struct acpi_rsdt_header *acpi_rsdt_header =
			(struct acpi_rsdt_header *)sdt_descriptor;
	for (int i = 0; i < 4; i++)
		printk("%c", acpi_rsdt_header->signature[i]);
	printk("\n");
	if (rsdt_checksum(acpi_rsdt_header) == true)
		return acpi_rsdt_header;
	else
		return (struct acpi_rsdt_header	*)-1;
	return 0;
}

struct rsdp_descriptor	*rdsp_stage(void)
{
	u8 *bios_addr = (u8 *)0xe0000;

	struct rsdp_descriptor *rsdp_descriptor = (struct rsdp_descriptor *)
			memschr(bios_addr, 0x20000, (u8 *)"RSD PTR ", 8, &rsdp_checksum);

	if (rsdp_descriptor != (struct rsdp_descriptor *)-1) {
		printk("ACPI RSDP_DESCRIPTOR founded !\n");
		printk("bios_ptr = %p version = %hhu oem_id = ", rsdp_descriptor, rsdp_descriptor->revision);
		for (int i = 0; i < 6; i++)
			printk("%c", rsdp_descriptor->oem_id[i]);
		printk("\n");
	}
	return rsdp_descriptor;
}

int	acpi(void)
{
	struct rsdp_descriptor *rsdp_descriptor = rdsp_stage();

	if (rsdp_descriptor == (struct rsdp_descriptor *)-1) {
		printk("rsdp_descriptor not founded ! ACPI seems absent\n");
		return -1;
	}

	struct acpi_rsdt_header	*acpi_rsdt_header;

	if (rsdp_descriptor->revision == 0) {
		// legacy RSDP descriptor
		acpi_rsdt_header = rsdt_stage((void *)rsdp_descriptor->rsdt_address);
	} else {
		// extended RSDP descriptor
		struct rsdp_descriptor_2 *s = (struct rsdp_descriptor_2 *)rsdp_descriptor;
		acpi_rsdt_header = rsdt_stage((void *)s->xsdt_address_0_31);
	}

	if (acpi_rsdt_header == (struct acpi_rsdt_header *)-1) {
		printk("rsdt_descriptor not founded ! ACPI critical error\n");
		return -1;
	}
	process_facp(acpi_rsdt_header);
	return 0;
}
