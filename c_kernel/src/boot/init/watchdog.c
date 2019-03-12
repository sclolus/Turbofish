
#include "libft.h"

#include "watchdog.h"

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
	eprintk("DOG CRITICAL ERROR !\n");
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
			eprintk("%s has changed at offset %x !\n", p.msg, i);
			while (1) {}
		}
	}
}

static u32 ro_sum = 0;

extern u8 start_kernel_watched_data;
extern u8 end_kernel_watched_data;

void	guard_all(void) {
	dog_guard(gdt);
	dog_guard(idt);
	dog_guard(ivt);
	// sum all data in Kernel and store result
	for (u8 *ptr = &start_kernel_watched_data; ptr < &end_kernel_watched_data; ptr++)
		ro_sum += (u32)*ptr;
}

void	check_all(void) {
	dog_bark(gdt);
	dog_bark(idt);
	dog_bark(ivt);
	u32 new_ro_sum = 0;
	// sum all data in Kernel and check if same result
	for (u8 *ptr = &start_kernel_watched_data; ptr < &end_kernel_watched_data; ptr++)
		new_ro_sum += (u32)*ptr;
	if (new_ro_sum != ro_sum) {
		eprintk("Kernel Read Only Data has changed ! Got %u instead of %u\n",
			new_ro_sum, ro_sum);
		while (1) {}
	}
}
