
#include "vesa_graphic.h"
#include "libft.h"
#include "base_system.h"

void	_main(struct graphic_ctx *graphic_ctx);

void	_start(void)
{
	struct graphic_ctx graphic_ctx;

	graphic_ctx.vesa_mode_info = (struct vesa_mode_info *)0x00008100;
	init_GDT(graphic_ctx.vesa_mode_info->framebuffer);

	// Reconfiguration of stack pointer SS:ESP assembly syntax AT&T
	// BE CAREFULL When redefinition of ESP, staked variables moved, so we need to redefine them
	asm("                           \
		movw $0x20, %ax\n           \
		movw %ax, %ss\n             \
		movl $0x9F000, %esp         \
	");

	graphic_ctx.vesa_global_info = (struct vesa_global_info *)0x00008000;
	graphic_ctx.vesa_mode_info = (struct vesa_mode_info *)0x00008100;
	graphic_ctx.vesa_graphic_mode_list = (struct vesa_graphic_mode_list *)0x00008200;

	_main(&graphic_ctx);
}
