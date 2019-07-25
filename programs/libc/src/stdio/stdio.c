#include "stdio.h"
#include "string.h"

FILE _stderr = { .fd =2 };
FILE *stderr = &_stderr;

FILE _stdout = { .fd =1 };
FILE *stdout = &_stdout;

FILE _stdin = { .fd = 0 };
FILE *stdin = &_stdin;

size_t fwrite(const void *ptr, size_t size, size_t nmemb, FILE *stream) {
	size_t i;
	for (i = 0; i < nmemb; i++) {
		write(stream->fd, ptr, size);
	}
	return i;
}

int fputc(int c, FILE *stream) {
	unsigned char char_to_write = (unsigned char)c;
	write(stream->fd, &char_to_write, 1);
	return (int)char_to_write;
}

int putc(int c, FILE *stream) {
	return fputc(c, stream);
}

int fputs(const char *s, FILE *stream) {
	write(stream->fd, s, strlen(s));
}

int ferror(FILE *stream) {
	return 0;
}

int puts(const char *s) {
	return fputs(s, stdout);
}
/* 
 * int      fputs(const char *restrict, FILE *restrict) {
 * }
 */

int      fflush(FILE *stream) {
	return 0;
}

int fclose(FILE *stream) {
	return 0;
}

int      putc_unlocked(int c, FILE *stream) {
	return putc(c, stream);
}

int putchar_unlocked(int c) {
	return putc(c, stdout);
}
