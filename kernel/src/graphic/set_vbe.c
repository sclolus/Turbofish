
#include "vesa_graphic.h"
#include "base_system.h"
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

u32 g_edi_offset = 0;

static ptr_32	*convert_to_linear_address(u16 segment, u16 offset)
{
	return (ptr_32 *)((segment << 4) + offset);
}

s32		set_vbe(u16 selected_mode)
{
	struct registers		reg;
	struct vesa_graphic_mode_list	*vgml;
	u16				*ptr;
	u32				i;

	// get global VBE info
	reg.eax = 0x4F00;
	reg.edi = VESA_GLOBAL_INFO_PTR;
	int8086(0x10, reg);

	memcpy(
		&g_graphic_ctx.vesa_global_info,
		(void *)VESA_GLOBAL_INFO_PTR,
		sizeof(struct vesa_global_info));

	// compute all VBE mode
	ptr = (u16 *)convert_to_linear_address(
		g_graphic_ctx.vesa_global_info.list_supported_mode_segment,
		g_graphic_ctx.vesa_global_info.list_supported_mode_offset);

	vgml = &g_graphic_ctx.vesa_graphic_mode_list;

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
	int8086(0x10, reg);

	memcpy(
		&g_graphic_ctx.vesa_mode_info,
		(void *)VESA_MODE_INFO_PTR,
		sizeof(struct vesa_mode_info));

	// needed by ASM PUTCHAR
	g_edi_offset = g_graphic_ctx.vesa_mode_info.width - 8;

	// re initialize GDT with Linear Frame Buffer address
	init_gdt(g_graphic_ctx.vesa_mode_info.framebuffer);

	// switch to selected graphic mode
	reg.eax = 0x4F02;
	reg.ebx = selected_mode | LFB_BIT;
	int8086(0x10, reg);

	return 0;
}
