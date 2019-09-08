#include <ltrace.h>
#include <libintl.h>

#warning LANGAGE TRANSLATIONS ARE ON A DUMMY STATE

/*
 * translate message
 */
char *gettext(const char *msgid)
{
	TRACE
	return (char *)msgid;
}
