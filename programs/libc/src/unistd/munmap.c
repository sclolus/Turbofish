#include "unistd.h"

extern int user_munmap(void *addr, size_t length);

int munmap(void *addr, size_t length)
{
	return user_munmap(addr, length);
}
