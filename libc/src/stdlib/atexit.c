#include <stdlib.h>

#warning ATEXIT FUNCTION MUST BE DEFINED
#include <custom.h>

int atexit(void(*f)(void))
{
	DUMMY
	(void)f;
	return 0;
}
