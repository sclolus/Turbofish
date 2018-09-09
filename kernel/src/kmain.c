
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

/*
	void *a = bmalloc(4096);
	ft_printf("a_addr = %p\n", a);
	void *b = bmalloc(4096);
	ft_printf("b_addr = %p\n", b);
	void *c = bmalloc(4096);
	ft_printf("c_addr = %p\n", c);
	void *d = bmalloc(8192);
	ft_printf("d_addr = %p\n", d);
	void *e = bmalloc(4096);
	ft_printf("e_addr = %p\n", e);
	void *f = bmalloc(8192);
	ft_printf("f_addr = %p\n", f);
	void *g = bmalloc(1 << 21);
	ft_printf("g_addr = %p\n", g);
	void *h = bmalloc(1 << 16);
	ft_printf("h_addr = %p\n", h);
	void *j = bmalloc(65536);
	ft_printf("j_addr = %p\n", j);

	ft_memset(a, 233, 4096);
	ft_printf("memset A done\n");
	ft_memset(b, 124, 4096);
	ft_printf("memset B done\n");
	ft_memset(c, 14, 4096);
	ft_printf("memset C done\n");
	ft_memset(d, 7, 8192);
	ft_printf("memset D done\n");
	ft_memset(e, 89, 4096);
	ft_printf("memset E done\n");
	ft_memset(f, 34, 8192);
	ft_printf("memset F done\n");
	ft_memset(g, 66, 1 << 21);
	ft_printf("memset G done\n");
	ft_memset(h, 22, 1 << 16);
	ft_printf("memset H done\n");
	ft_memset(j, 11, 65536);
	ft_printf("memset J done\n");

	int	test(u8 *addr, u8 c, size_t length)
	{
		for (size_t i = 0; i < length; i++)
		{
			if (addr[i] != c)
				return -1;
		}
		return 0;
	}

	test(a, 233, 4096);
	ft_printf("test A done\n");
	test(b, 124, 4096);
	ft_printf("test B done\n");
	test(c, 14, 4096);
	ft_printf("test C done\n");
	test(d, 7, 8192);
	ft_printf("test D done\n");
	test(e, 89, 4096);
	ft_printf("test E done\n");
	test(f, 34, 8192);
	ft_printf("test F done\n");
	test(g, 66, 1 << 21);
	ft_printf("test G done\n");
	test(h, 22, 1 << 16);
	ft_printf("test H done\n");
	test(j, 11, 65536);
	ft_printf("test J done\n");


	ft_memcpy(a, "bananes", 8);
	ft_printf("les %s sont cuites\n", a);

	ft_printf("free a: %i\n", bfree(a));
	ft_printf("free f: %i\n", bfree(f));
	ft_printf("free i: %i\n", bfree(j));
	ft_printf("free g: %i\n", bfree(g));
	ft_printf("free h: %i\n", bfree(h));
	ft_printf("free d: %i\n", bfree(d));
	ft_printf("free c: %i\n", bfree(c));
	ft_printf("free b: %i\n", bfree(b));
	ft_printf("free e: %i\n", bfree(e));

	ft_printf("frame count = %u\n", count_frames());
*/


	/*
	char *x = bmalloc(532 * 4096);
	ft_printf("ptr = %p\n", x);
	char *y = bmalloc(65521);
	ft_printf("ptr = %p\n", y);

	while (1);
	*/

	char *u = kmalloc(16);
	ft_printf("ptr = %p\n", u);
	ft_strcpy(u, "Sanglier");
	ft_printf("Le mot est %s\n", u);
	kfree(u);
	ft_printf("Le mot est %s\n", u);

	ft_printf("frame count = %u\n", count_frames());
	return;
}

