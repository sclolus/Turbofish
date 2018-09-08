
#include "i386_type.h"
#include "vesa_graphic.h"
#include "base_system.h"
#include "libft.h"
#include "grub.h"
#include "paging.h"

// this loops clears the screen
// there are 25 lines each of 80 columns; each element takes 2 bytes
void		reset_text_screen(void)
{
	struct registers	reg;
	char			*vidptr;
	u32			j;
	u32			screensize;

	// video memory begins at address 0xb8000
	vidptr = (char*)0xb8000;

	// set cursor 	AH=02h 	BH = page number, DH = Line, DL = Colomn
	reg.edx = 0;
	reg.ebx = 0;
	reg.eax = 0x02;
	int8086(0x10, reg);

	j = 0;
	screensize =  80 * 25 * 2;
	while (j < screensize)
	{
		vidptr[j] = ' ';	// black character
		vidptr[j + 1] = 0x07;	// attribute-byte
		j = j + 2;
	}
}

void		text_putstr(char *str)
{
	// video memory begins at address 0xb8000
	char *vidptr = (char*)0xb8000;
	static int i = 0;

	while (*str != '\0')
	{
		vidptr[i++] = *str++;	// the character's ascii
		vidptr[i++] = 0x07;	// give char black bg and light grey fg
	}
}

// for the moment, only mode in 8bpp work. 0x100 0x101 0x103 0x105 0x107
#define VBE_MODE 0x105

void 		kmain(struct multiboot_info *multiboot_info_addr)
{
	if (set_vbe(VBE_MODE) < 0)
	{
		reset_text_screen();
		text_putstr("KERNEL_FATAL_ERROR: Cannot set VBE mode");
		bios_wait(5);
		bios_shutdown_computer();
		return ;
	}

	ft_printf("{white}Kernel loaded: {green}OK\n{eoc}");
	ft_printf("{white}VBE initialized: {green}OK\n{eoc}");
	ft_printf("{white}GDT loaded: {green}OK\n{eoc}");
	ft_printf("{white}Available graphic mode:\n{eoc}");

	struct vesa_graphic_mode_list *vgml =
		&g_graphic_ctx.vesa_graphic_mode_list;

	ft_printf("{orange}");
	u32 max_cap = g_graphic_ctx.vesa_mode_info.width / 8 / 8;
	u32 i = 0;
	while (i < vgml->nb_mode)
	{
		ft_printf("0x%.4hx ", vgml->mode[i]);
		i++;
		ft_printf((i % max_cap == 0) ? "\n" : " ");
	}
	if (i % max_cap != 0)
		ft_printf("\n");
	ft_printf("{eoc}");

	ft_printf("{white}Selected mode: {green}%#x\n{eoc}", VBE_MODE);
	ft_printf("-> width: {green}%hu{eoc}, height:"
			" {green}%hu{eoc}, bpp: {green}%hhu{eoc}\n",
			g_graphic_ctx.vesa_mode_info.width,
			g_graphic_ctx.vesa_mode_info.height,
			g_graphic_ctx.vesa_mode_info.bpp);
	ft_printf("-> linear frame buffer location: {green}%#x{eoc}\n",
			g_graphic_ctx.vesa_mode_info.framebuffer);

	ft_printf("{white}Initialize IDT: ");
	init_IDT();
	ft_printf("{green}OK\n{eoc}");

	ft_printf("{white}Initialize PIC: ");
	init_PIC();
	ft_printf("{green}OK\n{eoc}");

	ft_printf("{white}Initialize Paging: ");
	init_paging();
	ft_printf("{green}OK\n{eoc}");

	ft_printf("flag: %p\n", multiboot_info_addr->flags);
	ft_printf("mem_lower: %u, mem_upper: %u\n", multiboot_info_addr->mem_lower, multiboot_info_addr->mem_upper);
	ft_printf("Addr = %p length = %u map_addr = %p\n", multiboot_info_addr, multiboot_info_addr->mmap_length, multiboot_info_addr->mmap_addr);

	asm("sti");
	ft_printf("{white}Interupt enabled: {green}OK{eoc}\n");

	set_text_color(1);
	putchar('H');
	set_text_color(2);
	putchar('E');
	set_text_color(3);
	putchar('L');
	set_text_color(4);
	putchar('L');
	set_text_color(5);
	putchar('O');
	putchar(' ');
	set_text_color(6);
	putchar('W');
	set_text_color(7);
	putchar('O');
	set_text_color(8);
	putchar('R');
	set_text_color(9);
	putchar('L');
	set_text_color(10);
	putchar('D');
	putchar('\n');

	init_frames();
	void *a = alloc_frames(1);
	ft_printf("a_addr = %p\n", a);
	void *b = alloc_frames(1);
	ft_printf("b_addr = %p\n", b);
	void *c = alloc_frames(1);
	ft_printf("c_addr = %p\n", c);
	void *d = alloc_frames(2);
	ft_printf("d_addr = %p\n", d);
	void *e = alloc_frames(1);
	ft_printf("e_addr = %p\n", e);
	void *f = alloc_frames(2);
	ft_printf("f_addr = %p\n", f);

	ft_printf("free a: %i\n", free_frames(a));
	ft_printf("free f: %i\n", free_frames(f));
	ft_printf("free d: %i\n", free_frames(d));
	ft_printf("free c: %i\n", free_frames(c));
	ft_printf("free b: %i\n", free_frames(b));
	ft_printf("free e: %i\n", free_frames(e));

	a = alloc_frames(1);
	ft_printf("a_addr = %p\n", a);
	b = alloc_frames(1);
	ft_printf("b_addr = %p\n", b);
	c = alloc_frames(1);
	ft_printf("c_addr = %p\n", c);
	d = alloc_frames(2);
	ft_printf("d_addr = %p\n", d);
	e = alloc_frames(1);
	ft_printf("e_addr = %p\n", e);
	f = alloc_frames(2);
	ft_printf("f_addr = %p\n", f);

	ft_printf("free a: %i\n", free_frames(a));
	ft_printf("free f: %i\n", free_frames(f));
	ft_printf("free d: %i\n", free_frames(d));
	ft_printf("free c: %i\n", free_frames(c));
	ft_printf("free b: %i\n", free_frames(b));
	ft_printf("free e: %i\n", free_frames(e));

	ft_printf("frame count = %u\n", count_frames());
	return;
}

