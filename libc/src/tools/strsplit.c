
#include <ltrace.h>
#include <stdlib.h>
#include <string.h>
#include <tools.h>

static uint32_t count_fields(char *const s, char del)
{
	TRACE
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

char **strsplit(char *const s, char del)
{
	TRACE
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
			for (uint32_t j = 0; j < i; j++) {
				free(fields[i]);
			}
			free(fields);
			return NULL;
		}
		fields[i][len] = '\0';
		memcpy(fields[i], start, len);
		u += len + 1;
	}
	return fields;
}
