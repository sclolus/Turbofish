
#include "i386_type.h"
#include "vesa_graphic.h"
#include "libft.h"

extern u8	get_keymap(u32 offset);

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

extern u32 pit_time_sec;
extern u32 pit_time_nsec;

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
		wrapper_1();
		break;
	}
	case 60: {
		int z;


		z = 42;
		z = z / 0;
		printk("value of z: %i\n", z);
		break;
	}
	case 61: {
		printk("time:%.4u.%.9u\n", pit_time_sec, pit_time_nsec);
		break;
	}
	default:
		if (scancode & 0x80)
			break;
		if (keyboard_register & MAJ)
			graphic_putchar(get_keymap((scancode << 2) + 1));
		else if (keyboard_register & ALT)
			graphic_putchar(get_keymap((scancode << 2) + 2));
		else
			graphic_putchar(get_keymap(scancode << 2));
		break;
	}
}
