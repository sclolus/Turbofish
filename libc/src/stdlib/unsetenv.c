#include <ltrace.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <errno.h>

#define __PRIVATE_USE_ENV_H__
# include "env.h"
#undef __PRIVATE_USE_ENV_H__

# define array_size __array_size

extern char **environ;

int unsetenv(const char *name)
{
	TRACE
	size_t	name_len = strlen(name);

	if (name_len == 0 || strchr(name, '=')) {
		errno = EINVAL;
		return -1;
	}

	if (!environ)
		return 0;

	char	**entry = search_env(name);

	if (!entry)
		return 0;

	free(*entry);

	size_t entry_pos = (size_t)(entry - environ);

	size_t environ_size = array_size((void *)environ);

	memmove(environ + entry_pos, environ + entry_pos + 1, environ_size - entry_pos - 1);
	environ[environ_size - 1] = NULL;
	return 0;
}

#undef array_size

#ifdef UNIT_TESTS
# include <criterion/criterion.h>

Test(unsetenv, basic) {
	environ = NULL;
	const char *key = "PATH";
	const char *value = "/usr/bin:/usr/local/bin";
	cr_assert_eq(setenv(key, value, true), 0);
	cr_assert_str_eq(getenv(key), value);

	unsetenv(key);
	cr_assert_eq(getenv(key), NULL);
}



Test(unsetenv, with_multiple_variables) {
	environ = NULL;
	const char *key = "PRESIDENT";
	const char *value = "DONALD_TRUMP(WTF)";
	cr_assert_eq(setenv("PATH", "/bin:/usr/bin:/usr/local/bin", true), 0);
	cr_assert_eq(setenv("SOMETHING", "ELSE", true), 0);
	cr_assert_eq(setenv(key, value, true), 0);
	cr_assert_eq(setenv("TERM", "asd;lfkjdl;fj", true), 0);
	cr_assert_str_eq(getenv(key), value);

	unsetenv(key);
	cr_assert_eq(getenv(key), NULL);
	cr_assert_str_eq(getenv("SOMETHING"), "ELSE");
	cr_assert_str_eq(getenv("TERM"), "asd;lfkjdl;fj");
}


Test(unsetenv, empty_env) {
	environ = NULL;

	cr_assert_eq(unsetenv("PATH"), 0);
	cr_assert_eq(getenv("PATH"), NULL);
}
#endif /* UNIT_TESTS */
