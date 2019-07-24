
#include "string.h"
#include "unistd.h"


void	putchar(char c)
{
	write(1, &c, 1);
}
