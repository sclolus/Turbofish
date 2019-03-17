
#include "libft.h"
#include "vga_text.h"

#include "early_watchdog.h"

static u8 circle_gdt[GDT_SIZE] = {0};
static u8 circle_idt[IDT_SIZE] = {0};
static u8 circle_ivt[IVT_SIZE] = {0};

struct params {
	enum dog dog;
	void *location;
	size_t size;
	u8 *circle;
	char *msg;
};

static struct params params[LEN] = {
		{gdt, (void *)GDT_AREA, GDT_SIZE, (u8 *)&circle_gdt, "GDT"},
		{idt, (void *)IDT_AREA, IDT_SIZE, (u8 *)&circle_idt, "IDT"},
		{ivt, (void *)IVT_AREA, IVT_SIZE, (u8 *)&circle_ivt, "IVT"},
};

_Noreturn static void critical_error(void) {
	set_text_color(yellow);
	eprintk("WATCHDOG CRITICAL ERROR: Init sequence halted\n");
	while(1) {}
}

 static struct params found_dog(enum dog dog) {
	for (int i = 0; i < LEN; i++) {
		if (params[i].dog == dog) {
			return params[i];
		}
	}
	critical_error();
}

void	dog_guard(enum dog dog) {
	struct params p = found_dog(dog);

	ft_memcpy((void *)p.circle, (const void *)p.location, p.size);
}

void	dog_bark(enum dog dog) {
	struct params p = found_dog(dog);

	for (size_t i = 0; i < p.size; i++) {
		if (p.circle[i] != (u8)((u8 *)p.location)[i]) {
			set_text_color(yellow);
			eprintk("%s has changed at offset %x !\n", p.msg, i);
			critical_error();
		}
	}
}

static u32 ro_checksum = 0;
static u32 ro_sum = 0;

extern u8 start_kernel_watched_data;
extern u8 end_kernel_watched_data;

/*
 * Fletcher's checksum: 8-bit implementation (32-bit checksum)
 * See https://en.wikipedia.org/wiki/Fletcher%27s_checksum
 */
u32 fletcher32(const u8 *data, size_t len)
{
	u32 c0, c1;

	for (c0 = c1 = 0; len >= 720; len -= 720) {
		for (u32 i = 0; i < 360; ++i) {
			c0 = c0 + *data++;
			c0 += *data++ << 8;
			c1 = c1 + c0;
		}
		c0 = c0 & 0xffff;
		c1 = c1 & 0xffff;
	}
	for (u32 i = 0; i < (len >> 1); ++i) {
		c0 = c0 + *data++;
		c0 += *data++ << 8;
		c1 = c1 + c0;
	}
	// If len is ODD
	if ((len & 1) == 1) {
		c0 = c0 + *data++;
		c1 = c1 + c0;
	}
	c0 = c0 & 0xffff;
	c1 = c1 & 0xffff;
	return (c1 << 16) | c0;
}

void	guard_all(void) {
	// Save the current GDT, IDT and IVT BIOS
	dog_guard(gdt);
	dog_guard(idt);
	dog_guard(ivt);

	// Checksum all data in Kernel and store result
	ro_checksum = fletcher32(
		&start_kernel_watched_data,
		&end_kernel_watched_data - &start_kernel_watched_data);

	// Sum all data in Kernel
	for (u8 *p = &start_kernel_watched_data; p < &end_kernel_watched_data; p++)
		ro_sum += *p;
}

void	check_all(void) {
	// Check if GDT, IDT, and IVT BIOS are the same as previous
	dog_bark(gdt);
	dog_bark(idt);
	dog_bark(ivt);

	// Checksum all data in Kernel, then check if same result as previous
	u32 new_ro_checksum = fletcher32(
		&start_kernel_watched_data,
		&end_kernel_watched_data - &start_kernel_watched_data);

	if (new_ro_checksum != ro_checksum) {
		set_text_color(yellow);
		eprintk("Kernel Read Only Data has changed !\n");
		eprintk("Checksum: Got %#.8x instead of %#.8x\n",
			new_ro_checksum, ro_checksum);
		critical_error();
	}

	// Sum all data in Kernel, then check if same result as previous
	u32 new_ro_sum = 0;
	for (u8 *p = &start_kernel_watched_data; p < &end_kernel_watched_data; p++)
		new_ro_sum += *p;

	if (new_ro_sum != ro_sum) {
		set_text_color(yellow);
		eprintk("Kernel Read Only Data has changed !\n");
		eprintk("Sum: Got %u instead of %u\n", new_ro_sum, ro_sum);
		critical_error();
	}
}
