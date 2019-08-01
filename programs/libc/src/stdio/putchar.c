#include <unistd.h>

int putchar(int c)
{
	return (int)write(1, &c, 1);
}
