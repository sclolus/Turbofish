#include <assert.h>
#include <sys/time.h>
#include <unistd.h>
#include <stdio.h>

int main(int argc, char **argv) {
	(void)argc;
	(void)argv;
	struct timeval	tv;
	struct timezone tz;

	while (42) {
		assert(gettimeofday(&tv, &tz) == 0);
		assert(gettimeofday(&tv, NULL) == 0);
		assert(gettimeofday(NULL, NULL) == 0);

		printf("Current unix time: %u\n", tv.tv_sec);
		sleep(1);
	}
}
