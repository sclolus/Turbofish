#include <locale.h>
#include <stddef.h>

#warning SETLOCALE FUNCTION MUST BE DEFINED
#include <custom.h>

char *setlocale(int category, const char *locale)
{
	DUMMY
	(void)category;
	(void)locale;
	return NULL;
}

#warning BINDTEXTDOMAIN FUNCTION MUST BE DEFINED

char *bindtextdomain(const char *domainname, const char *dirname)
{
	DUMMY
	(void)domainname;
	(void)dirname;
	return NULL;
}

#warning TEXTDOMAIN FUNCTION MUST BE DEFINED

char *textdomain(const char *domainname)
{
	DUMMY
	(void)domainname;
	return NULL;
}
