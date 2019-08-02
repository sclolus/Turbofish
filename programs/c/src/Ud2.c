
#include <unistd.h>
#include <stdio.h>

int main(void)
{
	printf("Initialise Ud2\n");
	asm("ud2");
	return 0;
}
