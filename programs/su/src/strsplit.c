#include <su.h>

static uint32_t    count_fields(char *const s, char del) {
	uint32_t count = 1;
	uint32_t i = 0;

	while (s[i]) {
		if (s[i] == del) {
			count++;
		}
		i++;
	}
	return count;
}

char			*strchrnul(const char *s, int c)
{
	char *ret = strchr(s, c);

	if (!ret) {
		return (char *)&s[strlen(s)];
	}
	return ret;
}

char			**strsplit(char *const s, char del)
{
	uint32_t    count = count_fields(s, del);
	char	    **fields = malloc(sizeof(char *) * (count + 1));

	if (!fields) {
		return NULL;
	}
	fields[count] = NULL;
	uint32_t u = 0;
	for (uint32_t i = 0; i < count; i++) {
		char	*start = s + u;
		char	*end = strchrnul(s + u, del);
		uint32_t len = (uint32_t)(end - start);

		if (!(fields[i] = malloc(len + 1))) {
			free(fields);
			// WARNING: This leaks
			return NULL;
		}

		fields[i][len] = '\0';
		memcpy(fields[i], start, len);
		u += len + 1;
	}

	return fields;
}

#ifdef UNIT_TESTS
# include <criterion/criterion.h>

	static uint32_t	count_str_array_size(char **const str) {
		uint32_t i = 0;
		while (str[i])
			i++;
		return i;
	}

	Test(_strsplit, basic) {
		char	*input = "a:b";
		char	del = ':';
		char	*expected[] = {
			"a",
			"b",
			NULL,
		};

		char	**res = strsplit(input, del);

		const uint32_t size = count_str_array_size(res);
		const uint32_t expected_size = count_str_array_size(expected);
		cr_assert_eq(size, expected_size);

		for (uint32_t i = 0; i < size; i++) {
			cr_assert(!strcmp(res[i], expected[i]));
		}
		free_array(res);

	}

	Test(_strsplit, basic_with_hole) {
		char	*input = "a:b:c:d::e";
		char	del = ':';
		char	*expected[] = {
			"a",
			"b",
			"c",
			"d",
			"",
			"e",
			NULL,
		};

		char	**res = strsplit(input, del);

		const uint32_t size = count_str_array_size(res);
		const uint32_t expected_size = count_str_array_size(expected);
		cr_assert_eq(size, expected_size);

		for (uint32_t i = 0; i < size; i++) {
			cr_assert(!strcmp(res[i], expected[i]));
		}
		free_array(res);

	}

	Test(_strsplit, single) {
		char	*input = "a";
		char	del = ':';
		char	*expected[] = {
			"a",
			NULL,
		};

		char	**res = strsplit(input, del);

		const uint32_t size = count_str_array_size(res);
		const uint32_t expected_size = count_str_array_size(expected);
		cr_assert_eq(size, expected_size);

		for (uint32_t i = 0; i < size; i++) {
			cr_assert(!strcmp(res[i], expected[i]));
		}
		free_array(res);

	}

#endif /* TESTS */
