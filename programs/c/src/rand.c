#include <stdlib.h>
#include <stdio.h>
#include <time.h>

#define bit_RDRND   (1 << 30)

int main(void)
{
	unsigned int eax;
	unsigned int ebx;
	unsigned int ecx;
	unsigned int edx;

	eax = 0x01;

	__asm__ __volatile__(
			     "cpuid;"
			     : "=a"(eax), "=b"(ebx), "=c"(ecx), "=d"(edx)
			     : "a"(eax)
			     );

	printf("The value of the ecx register is %08x.\n", ecx);

	if(ecx & bit_RDRND){
		//use rdrand
		printf("use rdrand\n");
	} else{
		//use mt19937
		printf("use mt19937\n");
	}

	for (int i = 0; i < 42; i++) {
		printf("%i ", rand());

		// struct timespec t;
		// t.tv_sec = 0;
		// t.tv_nsec = 1000000;
		// nanosleep(&t, NULL);
	}
	printf("\n");
	return 0;
}
