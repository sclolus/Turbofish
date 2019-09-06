#include <unistd.h>
#include <stdio.h>
#include <stdlib.h>
#include <assert.h>
#include <errno.h>

// This binary shall execute dash, that shall execute the `env` binary and sucessfully exit.
// The env binary should print the EXCL_TEST variable with a value of `working`.
int main(void)
{
	const char  *key = "EXCL_TEST";
	const char  *value = "working";

	assert(setenv(key, value, 1) == 0);
	printf("Set '%s' env variable to: '%s'\n", key, value);
	int ret = execl("/bin/dash", "dash", "-c", "/bin/env", (char*)NULL);

	assert(ret == -1);
	perror("Execl failed with: ");
	return EXIT_FAILURE;
}
