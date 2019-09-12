#include <ltrace.h>
#include <locale.h>
#include <stddef.h>

#warning SETLOCALE FUNCTION MUST BE DEFINED
#include <ltrace.h>
#include <custom.h>

char *setlocale(int category, const char *locale)
{
	TRACE
	DUMMY
	(void)category;
	(void)locale;
	return NULL;
}

#warning BINDTEXTDOMAIN FUNCTION MUST BE DEFINED

char *bindtextdomain(const char *domainname, const char *dirname)
{
	TRACE
	DUMMY
	(void)domainname;
	(void)dirname;
	return NULL;
}

#warning TEXTDOMAIN FUNCTION MUST BE DEFINED

char *textdomain(const char *domainname)
{
	TRACE
	DUMMY
	(void)domainname;
	return NULL;
}