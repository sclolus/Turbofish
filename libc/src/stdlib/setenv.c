#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include <errno.h>

#define __PRIVATE_USE_ENV_H__
# include "env.h"
#undef __PRIVATE_USE_ENV_H__

extern char **environ;

static size_t array_size(void **array) { // remove this
	size_t i = 0;

	while (array[i]) {
		i++;
	}
	return i;
}

char	**search_env(const char *envname)
{
	size_t envname_len = strlen(envname);

	if (envname_len == 0) {
		return NULL;
	}

	for (uint32_t i = 0; environ[i]; i++) {
		char *key_end = strchr(environ[i], '=');

		if (!key_end) {
			// if there is no '=', the entry is malformed, skip it.
			continue;
		}

		size_t	key_len = key_end - environ[i];

		if (key_len != envname_len) {
			continue;
		} else if (!strncmp(envname, environ[i], key_len)) {
			return &environ[i];
		}
	}
	return NULL;
}

char *make_env_entry(const char *envname, const char *envval)
{
	size_t	name_len = strlen(envname);
	size_t	val_len = strlen(envval);
	size_t	total_len = name_len + val_len + 1;

	char	*entry = malloc(total_len + 1);

	if (!entry) {
		return NULL;
	}

	entry[total_len] = '\0';
	memcpy(entry, envname, name_len);
	entry[name_len] = '=';
	memcpy(entry + name_len + 1, envval, val_len);
	return entry;
}

int32_t handle_null_environ(void)
{
	if (!environ) {
		char	**new = malloc(sizeof(char*));

		if (!new) {
			return -1;
		}

		new[0] = NULL;
		environ = new;
	}
	return 0;
}

int setenv(const char *envname, const char *envval, int overwrite)
{
	size_t envname_len = strlen(envname);

	if (envname_len == 0 || strchr(envname, '=')) {
		errno = EINVAL;
		return -1;
	}

	if (-1 == handle_null_environ())
		return -1;

	char	**entry = search_env(envname);

	if (!entry) {
		size_t env_size = array_size((void **)environ);
		size_t new_size = env_size + 1;

		char **new_env = realloc(environ, sizeof(char *) * (new_size + 1));

		if (!new_env) {

			// Should we set errno to ENOMEM explicitly ?
			return -1;
		}
		environ = new_env;
		environ[new_size] = NULL;
		environ[new_size - 1] = NULL;
		char	*new_entry = make_env_entry(envname, envval);

		if (!new_entry) {
			return -1;
		}

		environ[new_size - 1] = new_entry;

	} else if (overwrite) {
		char	*new_entry = make_env_entry(envname, envval);

		if (!new_entry) {
			return -1;
		}
		free(*entry);
		*entry = new_entry;
	}
	return 0;
}

#ifdef UNIT_TESTS
# include <criterion/criterion.h>

Test(make_env_entry, basic_a_b) {
	char	*key = "a";
	char	*value = "b";

	char	*entry = make_env_entry(key, value);
	cr_assert(entry);
	cr_assert_str_eq(entry, "a=b");
	free(entry);
}

Test(make_env_entry, basic_path) {
	char	*key = "PATH";
	char	*value = "/home/miniske/Android/Sdk/platform-tools/:/home/miniske/android-studio/bin::/home/miniske/.opam/default/bin:/home/miniske/.cargo/bin:/home/miniske/Android/Sdk/platform-tools/:/home/miniske/android-studio/bin::/usr/local/bin:/usr/bin:/bin:/usr/local/games:/usr/games:/home/miniske/android-toolchain/bin:/opt/android-ndk/toolchains/arm-linux-androideabi-4.9/prebuilt/linux-x86_64/:/home/miniske/.fzf/bin:/home/miniske/android-toolchain/bin:/opt/android-ndk/toolchains/arm-linux-androideabi-4.9/prebuilt/linux-x86_64/";

	char	*entry = make_env_entry(key, value);
	cr_assert(entry);
	cr_assert_str_eq(entry, "PATH=/home/miniske/Android/Sdk/platform-tools/:/home/miniske/android-studio/bin::/home/miniske/.opam/default/bin:/home/miniske/.cargo/bin:/home/miniske/Android/Sdk/platform-tools/:/home/miniske/android-studio/bin::/usr/local/bin:/usr/bin:/bin:/usr/local/games:/usr/games:/home/miniske/android-toolchain/bin:/opt/android-ndk/toolchains/arm-linux-androideabi-4.9/prebuilt/linux-x86_64/:/home/miniske/.fzf/bin:/home/miniske/android-toolchain/bin:/opt/android-ndk/toolchains/arm-linux-androideabi-4.9/prebuilt/linux-x86_64/");
	free(entry);
}

Test(setenv, basic_set_path) {
	environ = NULL;
	char	*key = "PATH";
	char	*value = "/usr/bin:/usr/local/bin:/usr/share/bin";
	int	ret = setenv(key, value, true);

	cr_assert_eq(ret, 0);

	char	*returned_value = getenv("PATH");
	cr_assert_str_eq(returned_value, value);
}

Test(setenv, null_environ) {
	environ = NULL;
	char	*key = "PATH";
	char	*value = "/usr/bin";

	int ret = setenv(key, value, true);
	cr_assert_eq(ret, 0);

	char	*returned_value = getenv("PATH");
	cr_assert_str_eq(returned_value, value);
}

Test(setenv, no_overwrite_doesnt_overwrite) {
	environ = NULL;
	char	*key = "PATH";
	char	*value = "/usr/bin";

	int ret = setenv(key, value, false);
	cr_assert_str_eq(getenv(key), value);
	cr_assert_eq(ret, 0);
	char	*new_value = "bullshit";

	ret = setenv(key, new_value, false);
	cr_assert_eq(ret, 0);

	char	*returned_value = getenv(key);
	cr_assert_str_eq(returned_value, value);
}


Test(setenv, overwrite_overwrites) {
	environ = NULL;
	char	*key = "PATH";
	char	*value = "/usr/bin";

	int ret = setenv(key, value, false);
	cr_assert_str_eq(getenv(key), value);
	cr_assert_eq(ret, 0);
	char	*new_value = "bullshit";

	ret = setenv(key, new_value, true);
	cr_assert_eq(ret, 0);

	char	*returned_value = getenv(key);
	cr_assert_str_eq(returned_value, new_value);
}

Test(setenv, posix_einval_envname_empty_string) {
	environ = NULL;
	char	*key = "";
	char	*value = "/usr/bin";

	int ret = setenv(key, value, true);
	cr_assert_eq(ret, -1);
	cr_assert_eq(errno, EINVAL);
}

Test(setenv, posix_einval_envname_contains_equals_char) {
	environ = NULL;
	char	*key = "PATH=";
	char	*value = "/usr/bin";

	int ret = setenv(key, value, true);
	cr_assert_eq(ret, -1);
	cr_assert_eq(errno, EINVAL);
}

#endif
