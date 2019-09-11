#include <math.h>

float       nanf(const char *tagp)
{
	return __builtin_nanf(tagp);
}
