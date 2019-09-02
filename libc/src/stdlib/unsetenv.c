#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <errno.h>

#define __PRIVATE_USE_ENV_H__
# include "env.h"
#undef __PRIVATE_USE_ENV_H__

extern char **environ;

int unsetenv(const char *name)
{
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

	size_t entry_pos = (size_t)(environ - entry);

	size_t environ_size = array_size((void *)environ);

	memmove(environ + entry_pos, environ + entry_pos + 1, environ_size - entry_pos - 1);
	environ[environ_size] = NULL;
	return 0;
}

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


#endif /* UNIT_TESTS */
