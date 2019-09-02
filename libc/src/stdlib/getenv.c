#include <errno.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>


#define __PRIVATE_USE_ENV_H__
# include "env.h"
#undef __PRIVATE_USE_ENV_H__

extern char **environ;

char *getenv(const char *name)
{
	if (!environ)
		return NULL;

	char	**entry = search_env(name);

	if (!entry) {
		return NULL;
	}
	return *entry;
}

#ifdef UNIT_TESTS
# include <criterion/criterion.h>

Test(getenv, basic_key_exists) {
	const char *key = "PATH";
	const char *value = "something lol";

	int ret = setenv(key, value, true);

	cr_assert_eq(ret, 0);

	char	*returned_value = getenv(key);

	cr_assert(returned_value);

	cr_assert_str_eq(value, returned_value);
}


Test(getenv, basic_key_does_not_exists) {
	const char *key = "PATH";
	cr_assert_eq(unsetenv(key), 0);

	char	*ret = getenv(key);

	cr_assert_eq(ret, NULL);
}

Test(getenv, null_environ) {
	environ = NULL;
	const char *key = "PATH";

	char	*ret = getenv(key);

	cr_assert_eq(ret, NULL);
}

Test(getenv, empty_string) {
	const char *key = "";

	char	*ret = getenv(key);

	cr_assert_eq(ret, NULL);
}


#endif /* UNIT_TESTS */
