#include <unistd.h>
#include <stdio.h>
#include <stdlib.h>
#include <assert.h>
#include <errno.h>
#include <string.h>

// This binary shall execute dash, that shall execute the `env` binary and sucessfully exit.
// The env binary should print the EXCL_TEST variable with a value of `working`.
int main(int ac, char **av)
{
	const char  *key = "EXCL_TEST";
	const char  *value = "working";

	char *previous_env = getenv(key);
	if (previous_env != NULL) {
		printf("%s\n", previous_env);
	} else {
		printf("NULL\n");
	}
	if (previous_env != NULL && strcmp(previous_env, value) == 0) {
		assert(ac == 2);
		assert(strcmp(av[0], "a") == 0);
		assert(strcmp(av[1], "b") == 0);
		return EXIT_SUCCESS;
	}
	assert(setenv(key, value, 1) == 0);
	printf("Set '%s' env variable to: '%s'\n", key, value);
	int ret = execl("/bin/execl/execl", "a", "b", (char*)NULL);

	assert(ret == -1);
	perror("Execl failed with: ");
	return EXIT_FAILURE;
}
