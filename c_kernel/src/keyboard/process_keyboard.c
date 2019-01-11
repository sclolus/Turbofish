
#include "i386_type.h"
#include "vesa.h"
#include "libft.h"
#include "system.h"
#include "kernel_io.h"

#define MAJ_SCANCODE	0x36
#define MAJ		0x1
#define ALT_SCANCODE	0x38
#define ALT		0x2

#define RELEASE_BIT	0x80

void	wrapper_2(void)
{
	int i = 3;
	(void)i;

	char *ptr = (char *)0x42424242;
	*ptr = 42;
}

void	wrapper_1(void)
{
	int i = 3;
	(void)i;

	wrapper_2();
}

void	process_keyboard(u8 scancode)
{
	static u32 keyboard_register = 0;

	switch (scancode) {
	case MAJ_SCANCODE:
		keyboard_register |= MAJ;
		break;
	case MAJ_SCANCODE | RELEASE_BIT:
		keyboard_register &= ~MAJ;
		break;
	case ALT_SCANCODE:
		keyboard_register |= ALT;
		break;
	case ALT_SCANCODE | RELEASE_BIT:
		keyboard_register &= ~ALT;
		break;
	case 224:
		printk("(special_char)");
		break;
	case 59: {
		select_tty(0);
		break;
	}
	case 60: {
		select_tty(1);
		break;
	}
	case 61: {
		select_tty(2);
		break;
	}
	case 62: {
		select_tty(3);
		break;
	}
	case 63: {
		struct timeval tv;
		clock_gettime(&tv);
		printk("time:%.4u.%.6u\n", tv.sec, tv.usec);
		break;
	}
	case 64: {
		wrapper_1();
		break;
	}
	case 65: {
		int z = 42;
		z -= 42;
		z = 3 / z;
		printk("value of z: %i\n", z);
		break;
	}
	default:
		if (scancode & 0x80)
			break;
/*
		if (keyboard_register & MAJ)
			graphic_putchar(get_keymap((scancode << 2) + 1));
		else if (keyboard_register & ALT)
			graphic_putchar(get_keymap((scancode << 2) + 2));
		else
			graphic_putchar(get_keymap(scancode << 2));
*/
		break;
	}
}
