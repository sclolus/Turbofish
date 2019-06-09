
#include <unistd.h>
#include <stdio.h>

int main(void)
{
	printf("My UID is %u\n", getuid());
	return 0;
}
