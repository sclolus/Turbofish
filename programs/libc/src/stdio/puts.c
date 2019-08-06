#include <stdio.h>
#include <string.h>

/*
 * puts() writes the string s and a trailing newline to stdout
 */
int puts(const char *s)
{
	size_t len = strlen(s);

	for (size_t i = 0; i < len; i++) {
		putchar(s[i]);
	}

	putchar('\n');
	return len;
}
