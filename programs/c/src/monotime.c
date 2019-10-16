#include <sys/time.h>
#include <time.h>
#include <stdio.h>

int main(void)
{
	struct timeval t;

	while (1) {
		get_monotonic_time(&t, NULL);
		printf("%i %i\n", t.tv_sec, t.tv_usec);

		struct timespec spec;
		spec.tv_sec = 0;
		spec.tv_nsec = 10000;
		nanosleep(&spec, NULL);
	}
	return 0;
}
