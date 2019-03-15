
#include "vesa.h"
#include "system.h"
#include "kernel_io.h"
#include "libft.h"

/*
GRUB MEMORY OCCUPATION
0x0      -> 0xFFF     BIOS_IDT, GDT et IDT
0x1000   -> 0x9FFFF   libre
0xA0000  -> 0xFFFFF   réservé pour le hardware
0x100000 -> 0x1FFFFF  réservé pour le noyau
0x200000 -> max libre
*/

#define VESA_GLOBAL_INFO_PTR		0x2000
#define VESA_MODE_INFO_PTR		0x2000

#define LFB_BIT (1 << 14)

/*
 * extern int alt__allocate_linear_frame_buffer(void *phy_addr, size_t len);
 */

static ptr_32	*convert_to_linear_address(u16 segment, u16 offset)
{
	return (ptr_32 *)((segment << 4) + offset);
}

s32		set_vbe(u16 selected_mode)
{
	struct base_registers		reg;
	struct vesa_graphic_mode_list	*vgml;
	u16				*ptr;
	u32				i;

	// get global VBE info
	reg.eax = 0x4F00;
	reg.edi = VESA_GLOBAL_INFO_PTR;
	_int8086(&reg, 0x10);

	ft_memcpy(
		&vesa_ctx.global_info,
		(void *)VESA_GLOBAL_INFO_PTR,
		sizeof(struct vesa_global_info));

	// compute all VBE mode
	ptr = (u16 *)convert_to_linear_address(
		vesa_ctx.global_info.list_supported_mode_segment,
		vesa_ctx.global_info.list_supported_mode_offset);

	vgml = &vesa_ctx.mode_list;

	vgml->nb_mode = 0;
	while (*ptr != 0xFFFF && vgml->nb_mode != MAX_NB_VESA_MODE)
		vgml->mode[(vgml->nb_mode)++] = *ptr++;

	// test if selected mode exist in VBE capability
	for (i = 0; i < vgml->nb_mode; i++) {
		if (selected_mode == vgml->mode[i])
			break ;
	}
	if (i == vgml->nb_mode)
		return -1;

	// get selected mode info include LFB address location
	reg.eax = 0x4F01;
	reg.ecx = selected_mode | LFB_BIT;	// CX 1 << 14 => LFB
	reg.edi = VESA_MODE_INFO_PTR;
	_int8086(&reg, 0x10);

	ft_memcpy(
		&vesa_ctx.mode,
		(void *)VESA_MODE_INFO_PTR,
		sizeof(struct vesa_mode_info));

	// needed by ASM PUTCHAR
	vesa_ctx.edi_offset = vesa_ctx.mode.pitch - vesa_ctx.mode.bpp;

	/*
	 * alt__allocate_linear_frame_buffer((void *)vesa_ctx.mode.framebuffer, vesa_ctx.mode.pitch * vesa_ctx.mode.height);
	 * vesa_ctx.mode.framebuffer = 0xf0000000;
	 */

	// re initialise GDT with Linear Frame Buffer address
	init_gdt(vesa_ctx.mode.framebuffer);

	// switch to selected graphic mode
	reg.eax = 0x4F02;
	reg.ebx = selected_mode | LFB_BIT;
	_int8086(&reg, 0x10);

	return 0;
}
