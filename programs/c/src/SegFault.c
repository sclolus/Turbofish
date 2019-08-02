
#include <unistd.h>
#include <stdio.h>

int main(void)
{
	printf("Initialise Segfault\n");
	char *s = (char *)0x42424242;
	*s = 42;
	return 0;
}
