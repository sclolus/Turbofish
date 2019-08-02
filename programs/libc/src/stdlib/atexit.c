#include <stdlib.h>

#include <custom.h>

#warning ATEXIT FUNCTION MUST BE DEFINED

int atexit(void (*f)(void)) {
	FUNC
	(void)f;
	return 0;
}
