
#include "i386_type.h"

struct symbol_entry {
	u32 offset;
	char type;
	const char *name;
};

#include "nm.map"

struct symbol {
	u32 offset;
	const char *name;
};

/*
 * Assuming that address of index entry are sorted
 */
struct symbol	_get_symbol(u32 eip)
{
	for (int i = 0; i < FN_DIR_LEN; i++) {
		if (eip < function_directory[i].offset) {
			if (i == 0)
				return (struct symbol){0, "trace error"};
			return (struct symbol){
				eip - function_directory[i - 1].offset,
				function_directory[i - 1].name};
		}
	}
	return (struct symbol){0, "???"};
}

static u32 *ebp;

void		_init_backtrace(u32 initial_ebp)
{
	ebp = (u32 *)initial_ebp;
}

u32		_get_eip(void)
{
	u32 eip = ebp[1];
	if (eip == 0)
		return 0;
	ebp = (u32 *)ebp[0];
	return eip;
}
