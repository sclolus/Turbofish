#include "unistd.h"

extern int user_mprotect(void *addr, size_t length, int prot);

int mprotect(void *addr, size_t length, int prot)
{
	return user_mprotect(addr, length, prot);
}
