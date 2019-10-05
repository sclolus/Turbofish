#include <unistd.h>
#include <stdlib.h>
#include <assert.h>
#include <errno.h>
#include <limits.h>
#include <string.h>

int main(void)
{
	char	expected[10] = "Turbofish";
	char	buf[HOST_NAME_MAX];

	assert(0 == sethostname(expected, sizeof (expected) - 1));
	memset(buf, 0, sizeof(buf));
	assert(0 == gethostname(buf, sizeof(buf)));
	assert(!strcmp("Turbofish", buf));

	memset(buf, 0, sizeof(buf));
	assert(0 == gethostname(buf, 1));
	assert(!memcmp("T", buf, 1));

	memset(buf, 0, sizeof(buf));
	assert(0 == gethostname(buf, 2));
	assert(!memcmp("Tu", buf, 2));

	memset(buf, 0, sizeof(buf));
	assert(0 == gethostname(buf, 10));
	assert(!memcmp("Turbofish", buf, 10));

	return EXIT_SUCCESS;
}
