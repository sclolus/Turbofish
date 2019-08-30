#include <stdio.h>
#include <stdlib.h>
#include <sys/types.h>

int main() {
	unsigned long long t = 4294967295;
	/* unsigned long long div = 100; */

	printf("%lu\n", sizeof(t));
	if (sizeof(t) != 8) {
		printf("long long must be 8 bytes long\n");
		exit(1);
	}
	printf("%llu\n", t+1);
	if ((t + 1) == 0) {
		printf("sizeof bullshited us, long long must be 8 bytes long\n");
		exit(1);
	}

	off_t o = 4294967295;
	if (sizeof(o) != 8) {
		printf("off_t must be 8 bytes long\n");
		exit(1);
	}
	printf("%llu\n", o+1);
	if ((o + 1) == 0) {
		printf("sizeof bullshited us, off_t must be 8 bytes long\n");
		exit(1);
	}
	off_t u = -1;
	if (!(u - 1 == -2)) {
		printf("off_t must be signed\n");
	}
}
