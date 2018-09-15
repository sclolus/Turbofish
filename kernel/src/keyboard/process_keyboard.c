
#include "i386_type.h"
#include "vesa_graphic.h"
#include "libft.h"

#include "kernel_io.h"

extern u8	get_keymap(u32 offset);

#define MAJ_SCANCODE	0x36
#define MAJ		0x1
#define ALT_SCANCODE	0x38
#define ALT		0x2

#define RELEASE_BIT	0x80

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
	case 59:
		set_kernel_io_mode();
		break;
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
