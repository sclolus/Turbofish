
#include "libft.h"

#include "watchdog.h"

#include "kernel_io.h"
#include "vesa.h"
#include "text_user_interface.h"

#define GDT_SIZE 0x800
#define IDT_SIZE 0x800

static u8 circle_gdt[GDT_SIZE] = {0};
static u8 circle_idt[IDT_SIZE] = {0};

struct params {
	enum dog dog;
	void *location;
	size_t size;
	u8 *circle;
	char *msg;
};

#define LEN 2

static struct params params[LEN] = {
		{gdt, (void *)0x800, GDT_SIZE, (u8 *)&circle_gdt, "GDT"},
		{idt, (void *)0x1000, IDT_SIZE, (u8 *)&circle_idt, "IDT"},
};

_Noreturn static void critical_error(void) {
	kernel_io_ctx.term_mode = panic_screen;
	set_cursor_location(1, 1);

	fill_window(0xFF, 0x00, 0x00);
	eprintk("DOG CRITICAL ERROR !\n");
	refresh_screen();
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
			kernel_io_ctx.term_mode = panic_screen;
			set_cursor_location(1, 1);

			fill_window(0xFF, 0x00, 0x00);
			eprintk("warning %s has changed !\n", p.msg);
			refresh_screen();
			while (1) {}
		}
	}
}
