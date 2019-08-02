#include <locale.h>
#include <stddef.h>

#include <custom.h>

#warning SETLOCALE FUNCTION MUST BE DEFINED

char *setlocale(int category, const char *locale)
{
	FUNC
	(void)category;
	(void)locale;
	return NULL;
}

#warning BINDTEXTDOMAIN FUNCTION MUST BE DEFINED

char *bindtextdomain(const char *domainname, const char *dirname)
{
	FUNC
	(void)domainname;
	(void)dirname;
	return NULL;
}

#warning TEXTDOMAIN FUNCTION MUST BE DEFINED

char *textdomain(const char *domainname)
{
	FUNC
	(void)domainname;
	return NULL;
}
