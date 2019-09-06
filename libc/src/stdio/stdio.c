#include <stdio.h>
#include <string.h>
#include <stdbool.h>
#include <unistd.h>
#include <custom.h>

FILE _stderr = { .fd = STDERR_FILENO, .eof = false, .error = false };
FILE *stderr = &_stderr;

FILE _stdout = { .fd = STDOUT_FILENO, .eof = false, .error = false  };
FILE *stdout = &_stdout;

FILE _stdin = { .fd = STDIN_FILENO, .eof = false, .error = false  };
FILE *stdin = &_stdin;

/*
 * The function fwrite() writes nmemb items of data, each size bytes long,
 * to the stream pointed to by stream, obtaining them from the location given by ptr.
 */
size_t fwrite(const void *ptr, size_t size, size_t nmemb, FILE *stream)
{
	size_t i;
	for (i = 0; i < nmemb; i++) {
		if (write(stream->fd, ptr, size) < 0) {
			stream->error = true;
			break;
		}
		ptr += size;
	}
	return i;
}

/*
 * putc() is equivalent to fputc() except that it may be implemented as a macro
 * which evaluates stream more than once.
 * return the character written as an unsigned char cast to an int or EOF on error.
 */
int putc(int c, FILE *stream)
{
	return fputc(c, stream);
}

/*
 * fputc() writes the character c, cast to an unsigned char, to stream.
 * return the character written as an unsigned char cast to an int or EOF on error
 */
int fputc(int c, FILE *stream)
{
	unsigned char char_to_write = (unsigned char)c;
	if (write(stream->fd, &char_to_write, 1) < 0) {
		stream->error = true;
		return EOF;
	} else {
		return (int)char_to_write;
	}
}

/*
 * puts() writes the string s and a trailing newline to stdout
 * return a nonnegative number on success, or EOF on error.
 */
int puts(const char *s)
{
	return fputs(s, stdout);
}

/*
 * fputs() writes the string s to stream, without its terminating null byte ('\0')
 * return a nonnegative number on success, or EOF on error.
 */
int fputs(const char *s, FILE *stream)
{
	if (write(stream->fd, s, strlen(s)) < 0) {
		stream->error = true;
		return EOF;
	} else {
		return 0;
	}
}

int ferror(FILE *stream)
{
	return (int)stream->error;
}

int feof(FILE *stream)
{
	return (int)stream->eof;
}


int fflush(FILE *stream)
{
	(void)stream;
	return 0;
}

int putc_unlocked(int c, FILE *stream)
{
	return putc(c, stream);
}

int putchar_unlocked(int c)
{
	return putchar(c);
}
