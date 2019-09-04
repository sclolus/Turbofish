#include <unistd.h>
#include <errno.h>
#include <user_syscall.h>

// The getcwd() function shall place an absolute pathname of the
// current working directory in the array pointed to by buf, and
// return buf. The pathname shall contain no components that are dot
// or dot-dot, or are symbolic links.

// If there are multiple pathnames that getcwd() could place in the
// array pointed to by buf, one beginning with a single <slash>
// character and one or more beginning with two <slash> characters,
// then getcwd() shall place the pathname beginning with a single
// <slash> character in the array. The pathname shall not contain any
// unnecessary <slash> characters after the leading one or two <slash>
// characters.

// The size argument is the size in bytes of the character array
// pointed to by the buf argument. If buf is a null pointer, the
// behavior of getcwd() is unspecified.

#warning NOT IMPLEMENTED
#include <custom.h>

char *getcwd(char *buf, size_t size)
{
	int ret = _user_syscall(GETCWD, 2, buf, size);
	if (ret < 0) {
		errno = -ret;
		return NULL;
	}
	return buf;
}
