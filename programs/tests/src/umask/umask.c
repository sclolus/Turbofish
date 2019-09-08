#include <sys/stat.h>
#include <stdlib.h>
#include <unistd.h>
#include <fcntl.h>
#include <stdio.h>
#include <assert.h>
#include <stdint.h>


int main(void)
{
	char	    filename[256];
	struct stat stats;
	// set umask to 0.
	umask(0);

	// Remember the old umask value to test umask return value.
	mode_t old = 0;
	for (uint32_t i = 0; i < 0777; i++) {
		printf("Testing umask: %u\n", i);
		mode_t new = (mode_t)i;
		mode_t ret = umask(new);

		assert(ret == old);

		snprintf(filename, sizeof(filename), "umask_creat_test_%u", i);

		int fd = open(filename, O_CREAT | O_EXCL, 0777);

		if (fd == -1) {
			perror("Opened failed");
		}
		// != -1.
		assert(fd > -1);
		assert(close(fd) == 0);
		assert(stat(filename, &stats) == 0);

		mode_t expected = 0777 & ~new;
		// Only interested in file bits permissions.
		mode_t got = stats.st_mode & 0777;

		assert(got == expected);
		assert(unlink(filename) == 0);
		old = new;
	}

	return EXIT_SUCCESS;
}
