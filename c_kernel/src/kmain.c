
#include "dynamic_allocator.h"
#include "kernel_io.h"
#include "i386_type.h"
#include "vesa.h"
#include "text_user_interface.h"
#include "system.h"
#include "libft.h"
#include "grub.h"
#include "tests.h"

/*
 * This benchmark use the PIT on IRQ0 to work
 */
u32		benchmark(void)
{
	static int count = 0;
	static u32 res = 0;
	static u32 sum;

	struct timeval tv;

	if (count == 0) {
		clock_gettime(&tv);
		sum = tv.sec * 1000000 + tv.usec;
		count++;
	} else {
		clock_gettime(&tv);
		u32 tmp = tv.sec * 1000000 + tv.usec;
		if ((tmp - sum) <= 1000000) {
			count++;
		} else {
			res = count;
			count = 0;
		}
	}
	return res;
}

/*
 * Background bitmap
 */
extern char _univers_bmp_start;
extern char _wanggle_bmp_start;

/*
 * For the moment, only mode in 24bpp and 32bpp 1024x768 mode work
 */
#define VBE_MODE	0x118

/*
 * Programmable Interrupt Timer frequency
 */
#define PIT_FREQUENCY	100

extern int _mmx_test(void);
extern int _sse1_sse2_test(void);
//extern int _avx_test(void);

extern u32 _align_stack(u32(*f)(), u32 args_len, ...);

/*
 * Main Kernel
 */
void 		kmain(struct multiboot_info *multiboot_info_addr, void *dev_map)
{
	(void)dev_map;
/*
 * Initialization sequence
 */
	/*
	 * Initialize Interrupt Descriptor Table
	 */
	init_idt();

	/*
	 * Set VBE mode
	 */
	if (set_vbe(VBE_MODE) < 0) {
		reset_text_screen();
		text_putstr("KERNEL_FATAL_ERROR: Cannot set VBE mode");
		bios_wait(5);
		bios_shutdown_computer();
		return ;
	}

	/*
	 * Initialize paging
	 */
	u32 avalaible_mem = (multiboot_info_addr->mem_upper + 1024) << 10;
	if (init_paging(avalaible_mem, &vesa_ctx.mode.framebuffer) == -1) {
		eprintk("KERNEL_FATAL_ERROR: Cannot set PAGING\n");
		refresh_screen();
		return ;
	}

	/*
	 * Initialize 8254 PIT, clock on IRQ0
	 */
	asm_pit_init(PIT_FREQUENCY);

	/*
	 * Initialize PIC, Hardware interrupt chip
	 */
	init_pic();

	/*
	 * load background images and create K_TTY
	 */
	init_kernel_io();

	int width;
	int height;
	u8 *img;
	img = bmp_load(
			(u8 *)&_univers_bmp_start,
			&width,
			&height,
			NULL);
	create_tty(img, 0xFFFFFF);
	create_tty(img, 0xFFFFFF);
	create_tty(img, 0xFFFFFF);

	img = bmp_load(
			(u8 *)&_wanggle_bmp_start,
			&width,
			&height,
			NULL);
	create_tty(img, 0xFF00FF);

	select_tty(0);

	/* ----------------------------------------------------- */
	asm("sti");

	printk("Kernel loaded: {green}OK{eoc}\n");
	printk("VBE initialized: {green}OK\n{eoc}");

	printk("GDT loaded: {green}OK\n{eoc}");

	printk("vesa signature: {green}%c%c%c%c{eoc}"
			" vbe version: {green}%hhx.%hhx{eoc}\n",
			vesa_ctx.global_info.vesa_Signature[0],
			vesa_ctx.global_info.vesa_Signature[1],
			vesa_ctx.global_info.vesa_Signature[2],
			vesa_ctx.global_info.vesa_Signature[3],
			(vesa_ctx.global_info.vesa_version >> 8) & 0xFF,
			vesa_ctx.global_info.vesa_version & 0xFF);

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

	printk("selected mode: {green}%#x\n{eoc}", VBE_MODE);
	printk("width: {green}%hu{eoc} height:"
			" {green}%hu{eoc} bpp: {green}%hhu{eoc}"
			" pitch: {green}%hu{eoc}\n",
			vesa_ctx.mode.width,
			vesa_ctx.mode.height,
			vesa_ctx.mode.bpp,
			vesa_ctx.mode.pitch);
	printk("linear frame buffer location: {green}%#x{eoc}\n",
			vesa_ctx.mode.framebuffer);

	printk("Initialize IDT: ");
	printk("{green}OK\n{eoc}");

	printk("Initialize PIC: ");
	printk("{green}OK\n{eoc}");

	printk("mmx test: ");
	if (_mmx_test() == 0)
		printk("{green}OK\n{eoc}");
	else
		printk("{red}FAIL\n{eoc}");

	printk("sse1 sse2 test: ");
	if (_align_stack((u32 (*)(void))(&_sse1_sse2_test), 8, 42, 31) == 11)
		printk("{green}OK\n{eoc}");
	else
		printk("{red}FAIL\n{eoc}");

	/*
	printk("avx test: ");
	if (_avx_test() == 0)
		printk("{green}OK\n{eoc}");
	else
		printk("{red}FAIL\n{eoc}");
	*/

	printk("Initialize Paging with %u ko of available memory: ",
			avalaible_mem >> 10);
	printk("{green}OK\n{eoc}");

	mem_test(k_family, 0);
	mem_test(v_family, 0);
	mem_test(k_sub_family, 0);

	printk("%u page fault has triggered\n", get_nb_page_fault());

	printk("Enable interupt: {green}OK{eoc}\n");

	printk("{yellow}H{green}E{cyan}L{red}L{magenta}O ");
	printk("{orange}W{white}O{yellow}R{deepblue}L{lightgreen}D{eoc}\n");

	printk("{yellow}TIP OF THE DAY:{eoc} Press F1 to F4 to change k_tty,"
		" F5 for the clock, and F6 and F7 to shake the kernel\n");

	/*
	 * Wait until next interrupt
	 */
	asm("sti\n"
	    "halt:\n"
	    "hlt\n"
	    "jmp halt");

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

