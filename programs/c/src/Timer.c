
#include <unistd.h>
#include <stdio.h>

int main(void)
{
	printf("Initialise Timer\n");
	while (1) {
		sleep(5);
		printf("five seconds elapsed\n");
	}
	return 0;
}
