
#include "memory_manager.h"
#include "kernel_io.h"
#include "i386_type.h"
#include "vesa_graphic.h"
#include "base_system.h"
#include "libft.h"
#include "grub.h"
#include "tests.h"

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
	while (j < screensize) {
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

	while (*str != '\0') {
		vidptr[i++] = *str++;	// the character's ascii
		vidptr[i++] = 0x07;	// give char black bg and light grey fg
	}
}

// for the moment, only mode in 8bpp work. 0x100 0x101 0x103 0x105 0x107
#define VBE_MODE 0x105

void 		kmain(struct multiboot_info *multiboot_info_addr)
{
	if (set_vbe(VBE_MODE) < 0) {
		reset_text_screen();
		text_putstr("KERNEL_FATAL_ERROR: Cannot set VBE mode");
		bios_wait(5);
		bios_shutdown_computer();
		return ;
	}
	init_kernel_io_ctx();

	printk("{white}Kernel loaded: {green}OK\n{eoc}");
	printk("{white}VBE initialized: {green}OK\n{eoc}");
	printk("{white}GDT loaded: {green}OK\n{eoc}");
	printk("{white}Available graphic mode:\n{eoc}");

	struct vesa_graphic_mode_list *vgml =
		&g_graphic_ctx.vesa_graphic_mode_list;

	printk("{orange}");
	u32 max_cap = g_graphic_ctx.vesa_mode_info.width / 8 / 8;
	u32 i = 0;
	while (i < vgml->nb_mode) {
		printk("0x%.4hx ", vgml->mode[i]);
		i++;
		printk((i % max_cap == 0) ? "\n" : " ");
	}
	if (i % max_cap != 0)
		printk("\n");
	printk("{eoc}");

	printk("{white}Selected mode: {green}%#x\n{eoc}", VBE_MODE);
	printk("-> width: {green}%hu{eoc}, height:"
			" {green}%hu{eoc}, bpp: {green}%hhu{eoc}\n",
			g_graphic_ctx.vesa_mode_info.width,
			g_graphic_ctx.vesa_mode_info.height,
			g_graphic_ctx.vesa_mode_info.bpp);
	printk("-> linear frame buffer location: {green}%#x{eoc}\n",
			g_graphic_ctx.vesa_mode_info.framebuffer);

	printk("{white}Initialize IDT: ");
	init_idt();
	printk("{green}OK\n{eoc}");

	printk("{white}Initialize PIC: ");
	init_pic();
	printk("{green}OK\n{eoc}");

	u32 avalaible_mem = (multiboot_info_addr->mem_upper + 1024) << 10;
	printk("{white}Initialize Paging with %u ko of available memory: ",
			avalaible_mem >> 10);
	init_paging(avalaible_mem);
	printk("{green}OK\n{eoc}");

	mem_test(k_family, 0);
	mem_test(v_family, 0);
	mem_test(k_sub_family, 0);

	printk("{white}Enable interupt: {green}OK{eoc}\n");

	printk("{yellow}H{green}E{cyan}L{red}L{magenta}O ");
	printk("{orange}W{white}O{yellow}R{deepblue}L{lightgreen}D{eoc}\n");

	set_kernel_io_mode();
	asm("sti");

	return;
}

/*
	printk("flag: %p\n", multiboot_info_addr->flags);
	printk("mem_lower: %u, mem_upper: %u\n",
			multiboot_info_addr->mem_lower,
			multiboot_info_addr->mem_upper);
	printk("Addr = %p length = %u map_addr = %p\n",
			multiboot_info_addr,
			multiboot_info_addr->mmap_length,
			multiboot_info_addr->mmap_addr);
*/

