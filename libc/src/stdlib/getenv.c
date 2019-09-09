#include <ltrace.h>
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
	TRACE
	if (!environ)
		return NULL;

	char	**entry = search_env(name);

	if (!entry) {
		return NULL;
	}

	char	*key = strchr(*entry, '=');

	if (!key) {
		return NULL;
	} else {
		return key + 1;
	}
}

#ifdef UNIT_TESTS
# include <criterion/criterion.h>

Test(getenv, basic_key_exists) {
	environ = NULL;
	const char *key = "PATH";
	const char *value = "something lol";

	int ret = setenv(key, value, true);

	cr_assert_eq(ret, 0);

	char	*returned_value = getenv(key);

	cr_assert(returned_value);

	cr_assert_str_eq(value, returned_value);
}

Test(getenv, with_multiple_variables) {
	environ = NULL;
	const char *key = "PATH";
	const char *value = "/bin/:/usr/bin:/usr/local/bin";

	cr_assert_eq(setenv("PATH", "/bin:/usr/bin:/usr/local/bin", true), 0);
	cr_assert_eq(setenv("SOMETHING", "ELSE", true), 0);
	cr_assert_eq(setenv(key, value, true), 0);
	cr_assert_eq(setenv("TERM", "asd;lfkjdl;fj", true), 0);
	cr_assert_str_eq(getenv(key), value);
}

Test(getenv, basic_key_does_not_exists) {
	environ = NULL;
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
