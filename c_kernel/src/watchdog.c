
#include "libft.h"

#include "watchdog.h"

#define LOW_MEMORY

#ifdef LOW_MEMORY
int alt_eprintk(const char *restrict format, ...);
#endif

#ifdef GRAPHIC
#include "kernel_io.h"
#include "vesa.h"
#include "text_user_interface.h"
#endif

#define GDT_SIZE 0x800
#define IDT_SIZE 0x800
#define BIOS_IDT_SIZE 0x400

static u8 circle_gdt[GDT_SIZE] = {0};
static u8 circle_idt[IDT_SIZE] = {0};
static u8 circle_idt_bios[BIOS_IDT_SIZE] = {0};

struct params {
	enum dog dog;
	void *location;
	size_t size;
	u8 *circle;
	char *msg;
};

#define LEN 3

static struct params params[LEN] = {
		{gdt, (void *)0x800, GDT_SIZE, (u8 *)&circle_gdt, "GDT"},
		{idt, (void *)0x1000, IDT_SIZE, (u8 *)&circle_idt, "IDT"},
		{idt_bios, (void *)0x0, BIOS_IDT_SIZE, (u8 *)&circle_idt_bios, "IDT BIOS"},
};

_Noreturn static void critical_error(void) {
#ifdef GRAPHIC
	kernel_io_ctx.term_mode = panic_screen;
	set_cursor_location(1, 1);

	fill_window(0xFF, 0x00, 0x00);
#endif
#ifdef LOW_MEMORY
	alt_eprintk("DOG CRITICAL ERROR !\n");
#else
	eprintk("DOG CRITICAL ERROR !\n");
#endif
#ifdef GRAPHIC
	refresh_screen();
#endif
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
#ifdef GRAPHIC
			kernel_io_ctx.term_mode = panic_screen;
			set_cursor_location(1, 1);

			fill_window(0xFF, 0x00, 0x00);
#endif

#ifdef LOW_MEMORY
			alt_eprintk("%s has changed at offset %x !\n", p.msg, i);
#else
			eprintk("warning %s has changed !\n", p.msg);
#endif
#ifdef GRAPHIC
			refresh_screen();
#endif
			while (1) {}
		}
	}
}
