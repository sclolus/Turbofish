
#include "i386_type.h"

struct symbol_entry {
	u32 offset;
	char type;
	const char *name;
};

#include "autobuild/nm.map"

struct symbol {
	u32 offset;
	const char *name;
};

/*
 * Assuming that address of index entry are sorted
 */
struct symbol	_get_symbol(u32 eip)
{
	int i = 0;
	while (i < FN_DIR_LEN) {
		if (eip < function_directory[i].offset) {
			if (i == 0)
				return (struct symbol){0, "trace error"};
			break;
		}
		i++;
	}
	return (struct symbol)
			{eip - function_directory[i - 1].offset,
			function_directory[i - 1].name};
}
