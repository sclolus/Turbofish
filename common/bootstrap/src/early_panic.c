
#include "early_panic.h"

#include "libft.h"

struct symbol_entry {
	u32 eip;
	char symbol;
	const char *function_name;
};

#include "autobuild/nm.map"

struct function_result {
	const char *s;
	u32 offset;
};
/*
 * Assuming that address of index entry are sorted
 */
static struct function_result get_function_name(u32 eip)
{
	struct function_result res;

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
static u32 trace(u32 ebp_value, u32 max_frame, u32 *eip_array)
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

#define TRACE_MAX 10

void cpu_panic_handler(const char *str, struct PanicRegisters r)
{
	printk("{red}Panic -> %s\n", str);
	printk("CS: %#0.2hx DS: %#0.2hx ES: %#0.2hx FS: %#0.2hx GS: %#0.2hx SS: %#0.2hx\n",
			r.cs, r.ds, r.es, r.fs, r.gs, r.ss);
	printk("EIP: %.8p EFLAGS: %.8p OLD_EBP: %.8p\n",
			r.eip, r.eflags, r.old_ebp);
	printk("EAX: %.8p EBX: %.8p ECX: %.8p EDX: %.8p\n",
			r.regs.eax, r.regs.ebx, r.regs.ecx, r.regs.edx);
	printk("ESI: %.8p EDI: %.8p EBP: %.8p ESP: %.8p\n",
			r.regs.esi, r.regs.edi, r.regs.ebp, r.regs.esp);

	/*
	 * stack trace
	 */
	struct function_result res;
	u32 eip_array[TRACE_MAX];
	u32 trace_size;

	printk("dumping core:\n");

	res = get_function_name(r.eip);
	printk("%p  [EIP - %#.4x]  %s\n", r.eip, res.offset, res.s);

	trace_size = trace(r.old_ebp, TRACE_MAX, eip_array);

	for (u32 i = 0; i < trace_size; i++) {
		res = get_function_name(eip_array[i]);
		printk("%p  [EIP - %#.4x]  %s\n",
				eip_array[i], res.offset, res.s);
	}

	while (1) {}
}
