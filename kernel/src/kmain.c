
#include "dynamic_allocator.h"
#include "memory/memory_manager.h"

#include "kernel_io.h"
#include "i386_type.h"
#include "vesa_graphic.h"
#include "system.h"
#include "libft.h"
#include "grub.h"
#include "tests.h"

// this loops clears the screen
// there are 25 lines each of 80 columns; each element takes 2 bytes
void		reset_text_screen(void)
{
	struct base_registers	reg;
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

// for the moment, only mode in 24bpp and 32bpp 1024x768 mode work
#define VBE_MODE	0x118

#define PIT_FREQUENCY	1433

extern char _binary_medias_univers_bmp_start;

void 		kmain(struct multiboot_info *multiboot_info_addr)
{
	if (set_vbe(VBE_MODE) < 0) {
		reset_text_screen();
		text_putstr("KERNEL_FATAL_ERROR: Cannot set VBE mode");
		bios_wait(5);
		bios_shutdown_computer();
		return ;
	}
	init_idt();

	u32 avalaible_mem = (multiboot_info_addr->mem_upper + 1024) << 10;
	init_paging(avalaible_mem);

	asm_pit_init(PIT_FREQUENCY);
	init_pic();

	int width;
	int height;
	bmp_load(
			(u8 *)&_binary_medias_univers_bmp_start,
			&width,
			&height,
			NULL);

	g_kernel_io_ctx.term_mode = boot;

	printk("{white}Kernel loaded: {green}OK\n{eoc}");
	printk("{white}VBE initialized: {green}OK\n{eoc}");
	printk("{white}GDT loaded: {green}OK\n{eoc}");
	printk("{white}Available graphic mode:\n{eoc}");

	struct vesa_graphic_mode_list *vgml =
		&vesa_ctx.mode_list;

	printk("{orange}");
	u32 max_cap = vesa_ctx.mode.width / 8 / 8;
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
			" {green}%hu{eoc}, bpp: {green}%hhu{eoc}"
			" pitch: {green}%hu{eoc}\n",
			vesa_ctx.mode.width,
			vesa_ctx.mode.height,
			vesa_ctx.mode.bpp,
			vesa_ctx.mode.pitch);
	printk("-> linear frame buffer location: {green}%#x{eoc}\n",
			vesa_ctx.mode.framebuffer);

	printk("{white}Initialize IDT: ");
//	init_idt();
	printk("{green}OK\n{eoc}");

	printk("{white}Initialize PIC: ");
//	init_pic();
	printk("{green}OK\n{eoc}");

	printk("{white}Initialize Paging with %u ko of available memory: ",
			avalaible_mem >> 10);

/*
	if (init_paging(avalaible_mem) == -1) {
		printk("{red}FAIL\n{eoc}");
		return ;
	}
*/
	printk("{green}OK\n{eoc}");


	mem_test(k_family, 0);
	mem_test(v_family, 0);
	mem_test(k_sub_family, 0);

	printk("%u page fault has triggered\n", get_nb_page_fault());

	printk("{white}Enable interupt: {green}OK{eoc}\n");

	printk("{yellow}H{green}E{cyan}L{red}L{magenta}O ");
	printk("{orange}W{white}O{yellow}R{deepblue}L{lightgreen}D{eoc}\n");

	printk("{yellow}TIP OF THE DAY:{eoc} Press F1 or F2 to shake the kernel"
		", F3 for clock\n");

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

