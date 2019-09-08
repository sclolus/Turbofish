#include <ltrace.h>
#include <stdio.h>
#include <fcntl.h>
#include <errno.h>
#include <strings.h>
#include <stdlib.h>

struct _flags {
	const char *string;
	const int flags;
};

#define NBR_COMB_FLAGS 15

static const struct _flags flags[NBR_COMB_FLAGS] = {
	{ "r", O_RDONLY },
	{ "rb", O_RDONLY },
	{ "w", O_WRONLY | O_CREAT | O_TRUNC },
	{ "wb", O_WRONLY | O_CREAT | O_TRUNC },
	{ "a", O_WRONLY | O_CREAT | O_APPEND },
	{ "ab", O_WRONLY | O_CREAT | O_APPEND },
	{ "r+", O_RDWR },
	{ "rb+", O_RDWR },
	{ "r+b", O_RDWR },
	{ "w+", O_RDWR | O_CREAT | O_TRUNC },
	{ "wb+", O_RDWR | O_CREAT | O_TRUNC },
	{ "w+b", O_RDWR | O_CREAT | O_TRUNC },
	{ "a+", O_RDWR | O_CREAT | O_APPEND },
	{ "ab+", O_RDWR | O_CREAT | O_APPEND },
	{ "a+b", O_RDWR | O_CREAT | O_APPEND },
};

/*
 * stream open functions
 */
FILE *fopen(const char *restrict pathname, const char *restrict mode)
{
	TRACE
	if (pathname == NULL || mode == NULL) {
		errno = EINVAL;
		return NULL;
	}
	for (int i = 0; i < NBR_COMB_FLAGS; i++) {
		/* Windows Devs like to use Major Case for mode */
		if (strcasecmp(flags[i].string, mode) == 0) {
			int filemode = (flags[i].flags & O_CREAT) ?
				S_IRUSR | S_IWUSR | S_IRGRP | S_IWGRP | S_IROTH | S_IWOTH :
				0;
			int fd = open(pathname, flags[i].flags, filemode);
			if (fd < 0) {
				errno = -fd;
				return NULL;
			}
			FILE *stream = (FILE *)calloc(sizeof(FILE), 1);
			if (stream == NULL) {
				close(fd);
				return NULL;
			}
			/*
			 * Upon successful completion fopen() return a FILE pointer.
			 * Otherwise, NULL is returned and errno is set to indicate the error.
			 */
			stream->fd = fd;
			return stream;
		}
	}
	errno = EINVAL;
	return NULL;
}

#undef NBR_COMB_FLAGS
