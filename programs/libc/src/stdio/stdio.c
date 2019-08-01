#include <stdio.h>
#include <string.h>
#include <unistd.h>

FILE _stderr = { .fd =2 };
FILE *stderr = &_stderr;

FILE _stdout = { .fd =1 };
FILE *stdout = &_stdout;

FILE _stdin = { .fd = 0 };
FILE *stdin = &_stdin;

/*
 * The function fwrite() writes nmemb items of data, each size bytes long,
 * to the stream pointed to by stream, obtaining them from the location given by ptr.
 */
size_t fwrite(const void *ptr, size_t size, size_t nmemb, FILE *stream) {
	size_t i;
	for (i = 0; i < nmemb; i++) {
		if (write(stream->fd, ptr, size) < 0) {
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
int putc(int c, FILE *stream) {
	return fputc(c, stream);
}

/*
 * fputc() writes the character c, cast to an unsigned char, to stream.
 * return the character written as an unsigned char cast to an int or EOF on error
 */
int fputc(int c, FILE *stream) {
	unsigned char char_to_write = (unsigned char)c;
	if (write(stream->fd, &char_to_write, 1) < 0) {
		return EOF;
	} else {
		return (int)char_to_write;
	}
}

/*
 * puts() writes the string s and a trailing newline to stdout
 * return a nonnegative number on success, or EOF on error.
 */
int puts(const char *s) {
	return fputs(s, stdout);
}

/*
 * fputs() writes the string s to stream, without its terminating null byte ('\0')
 * return a nonnegative number on success, or EOF on error.
 */
int fputs(const char *s, FILE *stream) {
	if (write(stream->fd, s, strlen(s)) < 0 || write(stream->fd, "\n", 1) < 0) {
		return EOF;
	} else {
		return 0;
	}
}

#define DEBUG printf("%s called", __func__);

int ferror(FILE *stream) {
	(void)stream;
	DEBUG
	return 0;
}

int fflush(FILE *stream) {
	(void)stream;
	DEBUG
	return 0;
}

int fclose(FILE *stream) {
	(void)stream;
	DEBUG
	return 0;
}

int putc_unlocked(int c, FILE *stream) {
	DEBUG
	return putc(c, stream);
}

int putchar_unlocked(int c) {
	DEBUG
	return putc(c, stdout);
}
