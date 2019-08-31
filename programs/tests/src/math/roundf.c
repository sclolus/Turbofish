#include <math.h>
#include <stdio.h>
#include <unistd.h>
#include <stdlib.h>
#include <errno.h>
#include <string.h>

struct result {
	float f;
	int s;
};

#define NB_TESTS 4

struct result result[NB_TESTS] = {{3.14, 3}, {-3.14, -3}, {3.98, 4}, {-3.52, -4}};

int main(void)
{
	for (int i = 0; i < 10; i++) {
		for (int j = 0; j < NB_TESTS; j++) {
			int s = (int)roundf(result[j].f);
			if (s != result[j].s) {
				printf("roundf must be %i, got %i\n", result[j].s, s);
				exit(1);
			}
		}
		printf("FPU-ROUNDF test done: %i\n", i);
		usleep(2500);
	}
	return 0;
}
