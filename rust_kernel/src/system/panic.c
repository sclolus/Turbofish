
#include "i386_type.h"

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
