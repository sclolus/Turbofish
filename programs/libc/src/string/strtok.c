#include <string.h>
/* Parse S into tokens separated by characters in DELIM.
   If S is NULL, the saved pointer in SAVE_PTR is used as
   the next starting point.  For example:
   char s[] = "-abc-=-def";
   char *sp;
   x = strtok_r(s, "-", &sp);        // x = "abc", sp = "=-def"
   x = strtok_r(NULL, "-=", &sp);        // x = "def", sp = NULL
   x = strtok_r(NULL, "=", &sp);        // x = NULL
   // s = "abc\0-def\0"
   */
static char *__strtok(char *s, const char *delim, char **save_ptr)
{
	char *end;
	if (s == NULL)
		s = *save_ptr;
	if (*s == '\0')
	{
		*save_ptr = s;
		return NULL;
	}
	/* Scan leading delimiters.  */
	s += strspn(s, delim);
	if (*s == '\0')
	{
		*save_ptr = s;
		return NULL;
	}
	/* Find the end of the token.  */
	end = s + strcspn(s, delim);
	if (*end == '\0')
	{
		*save_ptr = end;
		return s;
	}
	/* Terminate the token and make *SAVE_PTR point past it.  */
	*end = '\0';
	*save_ptr = end + 1;
	return s;
}

char *strtok(char *s, const char *delim) {
	static char *olds;
	__strtok(s, delim, &olds);
}
