#include <wchar.h>
#include <string.h>
#include <errno.h>

size_t mbrtowc(wchar_t *pwc, const char *s, size_t n, mbstate_t *ps)
{
	errno = ENOSYS;
	return (size_t)-1;
}
