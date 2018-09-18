
#include "base_system.h"

#include "i386_type.h"
#include "libft.h"
#include "vesa_graphic.h"

struct function_entry {
	u32 eip;
	char symbol;
	const char *function_name;
};

#include "nm.map"

struct function_result {
	const char *s;
	u32 offset;
};
/*
 * Assuming that address of index entry are sorted
 */
static struct function_result	get_function_name(u32 eip)
{
	struct function_result	res;

	for (int i = 0; i < FN_DIR_LEN; i++) {
		if (eip < function_directory[i].eip) {
			if (i == 0) {
				res.s = "trace error";
				res.offset = 0;
			}
			res.s =  function_directory[i - 1].function_name;
			res.offset = eip - function_directory[i - 1].eip;
			return res;
		}
	}
	res.s = "???";
	res.offset = 0;
	return res;
}


/*
 * each function store the EBP of the previous function into stack
 * push EBP		push EBP of the previous function
 * mov ebp, esp		set his own EBP, it's in the top of the stack (esp)
 * And just before EBP, we can found the EIP of the caller
 *
 * EBP_2 EIP_2 param param param EBP_1 EIP_1 param param EBO_0 EIP_0
 *
 * The stack contain
 * Second function argument (eip_array pointer)
 * First function argument (max_frame)
 * Return address in calling function
 * EBP of calling function (pointed to by current EBP)
 */
static u32		trace(u32 ebp_value, u32 max_frame, u32 *eip_array)
{
	u32 *ebp;
	u32 eip;
	u32 frame;

	ebp = (u32 *)ebp_value;

	frame = 0;
	for (frame = 0; frame < max_frame; frame++) {
		/*
		 * Access to EBP + 1 => EIP
		 */
		eip = ebp[1];
		if (eip == 0)
			break;
		/*
		 * Unwind to previous stack frame
		 */
		ebp = (u32 *)ebp[0];

		/*
		 * store the EIP address found
		 */
		*eip_array++ = eip;
	}
	return frame;
}

#define TRACE_MAX	10

extern void exit_panic(void);

void	panic(const char *s, struct extended_registers reg)
{
	memset4((u32 *)g_graphic_ctx.vesa_mode_info.framebuffer,
		0x01010101,
		(g_graphic_ctx.vesa_mode_info.width *
		g_graphic_ctx.vesa_mode_info.height) >> 2);

	struct function_result	res;
	u32			colomn;
	u32			line;
	u32			eip_array[TRACE_MAX];
	u32			trace_size;

	colomn = 40;
	line = 10;

	set_text_color(7);

	set_cursor_location(colomn + 20, line);
	eprintk("KFS");

	set_cursor_location(colomn, line + 2);
	eprintk("An error has occurred: KERNEL PANIC");

	set_cursor_location(colomn, line + 4);
	eprintk("%s", s);

	set_cursor_location(colomn, line + 6);
	eprintk("EAX: 0x%.8x  EBP: 0x%.8x  eflags: 0x%.8x",
			reg.eax, reg.old_ebp, reg.eflags);

	set_cursor_location(colomn, line + 7);
	eprintk("EBX: 0x%.8x  ESP: 0x%.8x  SS: 0x%.4hx",
			reg.ebx, reg.esp, reg.ss);

	set_cursor_location(colomn, line + 8);
	eprintk("ECX: 0x%.8x  ESI: 0x%.8x  DS: 0x%.4hx",
			reg.ecx, reg.esi, reg.ds);

	set_cursor_location(colomn, line + 9);
	eprintk("EDX: 0x%.8x  EDI: 0x%.8x  ES: 0x%.4hx",
			reg.edx, reg.edi, reg.es);

	set_cursor_location(colomn + 17, line + 10);
	eprintk("EIP: 0x%.8x  CS: 0x%.4hx", reg.eip, reg.cs);

	/*
	 * stack trace
	 */
	set_cursor_location(colomn, line + 12);
	printk("dumping core:");

	set_cursor_location(colomn, line + 14);
	res = get_function_name(reg.eip);
	printk("%p  [EIP - %#.4x]  %s", reg.eip, res.offset, res.s);

	trace_size = trace(reg.old_ebp, TRACE_MAX, eip_array);

	for (u32 i = 0; i < trace_size; i++) {
		set_cursor_location(colomn, line + 15 + i);
		res = get_function_name(eip_array[i]);
		printk("%p  [EIP - %#.4x]  %s",
				eip_array[i], res.offset, res.s);
	}

	set_cursor_location(colomn + 7, line + 27);
	eprintk("You can reboot your computer");

	exit_panic();

	asm("sti\n"
	    "loop:\n"
	    "hlt\n"
	    "jmp loop");
}
