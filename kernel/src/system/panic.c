
#include "base_system.h"

#include "i386_type.h"
#include "libft.h"
#include "vesa_graphic.h"

void	panic(
		const char *s,
		struct registers reg,
		u16 ds,
		u16 es,
		u16 ss,
		u32 eflags,
		u16 cs,
		u32 eip)
{
	memset4(
			(u32 *)g_graphic_ctx.vesa_mode_info.framebuffer,
			0x01010101,
			(g_graphic_ctx.vesa_mode_info.width *
			g_graphic_ctx.vesa_mode_info.height) >> 2);

	u32 colomn;
	u32 line;

	colomn = 40;
	line = 15;

	set_text_color(7);
	set_cursor_location(colomn + 20, line);
	eprintk("KFS\n\n");
	set_cursor_location(colomn, line + 2);
	eprintk("An error has occurred: PANIC\n\n");

	set_cursor_location(colomn, line + 4);
	eprintk("%s\n\n", s);

	set_cursor_location(colomn, line + 6);
	eprintk("EAX: %#0.8x  EBP: %#0.8x  eflags: %#0.8x\n",
			reg.eax, reg.ebp, eflags);

	set_cursor_location(colomn, line + 7);
	eprintk("EBX: %#0.8x  ESP: %#0.8x  SS: %#0.4hx\n",
			reg.ebx, reg.esp, ss);

	set_cursor_location(colomn, line + 8);
	eprintk("ECX: %#0.8x  ESI: %#0.8x  DS: %#0.4hx\n",
			reg.ecx, reg.esi, ds);

	set_cursor_location(colomn, line + 9);
	eprintk("EDX: %#0.8x  EDI: %#0.8x  ES: %#0.4hx\n",
			reg.edx, reg.edi, es);

	set_cursor_location(colomn, line + 10);
	eprintk("                 EIP: %#0.8x  CS: %#0.4hx\n", eip, cs);

	set_cursor_location(colomn + 7, line + 12);
	eprintk("You can reboot your computer");

	asm("cli\n"
	    "loop:\n"
	    "hlt\n"
	    "jmp loop");
}
