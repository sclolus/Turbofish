#ifndef STDIO_H
# define STDIO_H

#include <stdarg.h>
#include <stddef.h>

//[CX] [Option Start] Some of the functionality described on this reference page extends the ISO C standard. Applications shall define the appropriate feature test macro (see XSH The Compilation Environment ) to enable the visibility of these symbols in this header. [Option End]

//The <stdio.h> header shall define the following data types through typedef:

// A non-array type containing all information needed to specify uniquely every position within a file.

typedef size_t fpos_t;

// A structure containing information about a file.
typedef struct {
	int             fd;
	fpos_t          offset;
	unsigned char   eof : 1,
                        error: 1;
} FILE;

//off_t
//    As described in <sys/types.h>.
#include <sys/types.h>
//size_t
//    As described in <stddef.h>.
//ssize_t
//    [CX] [Option Start] As described in <sys/types.h>. [Option End]
//va_list
#include <stdarg.h>
//    [CX] [Option Start] As described in <stdarg.h>. [Option End]


//The <stdio.h> header shall define the following macros which shall expand to integer constant expressions:

#define BUFSIZ 1
	//Size of <stdio.h> buffers. [CX] [Option Start]  This shall expand to a positive value. [Option End]
//L_ctermid
//    [CX] [Option Start] Maximum size of character array to hold ctermid() output. [Option End]
//L_tmpnam
//    [OB] [Option Start] Maximum size of character array to hold tmpnam() output. [Option End]

//The <stdio.h> header shall define the following macros which shall expand to integer constant expressions with distinct values:

//_IOFBF
//    Input/output fully buffered.
//_IOLBF
//    Input/output line buffered.
//_IONBF
//    Input/output unbuffered.

//The <stdio.h> header shall define the following macros which shall expand to integer constant expressions with distinct values:

#define SEEK_CUR 1
//   Seek relative to current position.
#define SEEK_END 2
//   Seek relative to end-of-file.
#define SEEK_SET 3
//    Seek relative to start-of-file.

//The <stdio.h> header shall define the following macros which shall expand to integer constant expressions denoting implementation limits:
//
//{FILENAME_MAX}
//    Maximum size in bytes of the longest pathname that the implementation guarantees can be opened.
//{FOPEN_MAX}
//    Number of streams which the implementation guarantees can be open simultaneously. The value is at least eight.
//{TMP_MAX}
//    [OB] [Option Start] Minimum number of unique filenames generated by tmpnam(). Maximum number of times an application can call tmpnam() reliably. The value of {TMP_MAX} is at least 25. [Option End]
//
//    [OB XSI] [Option Start] On XSI-conformant systems, the value of {TMP_MAX} is at least 10000. [Option End]
//
//The <stdio.h> header shall define the following macro which shall expand to an integer constant expression with type int and a negative value:
#ifndef EOF
# define EOF (-1)
#endif
//    End-of-file return value.
//
//The <stdio.h> header shall define NULL as described in <stddef.h>.
//
//The <stdio.h> header shall define the following macro which shall expand to a string constant:
//
//P_tmpdir
//    [OB XSI] [Option Start] Default directory prefix for tempnam(). [Option End]
//
//The <stdio.h> header shall define the following macros which shall expand to expressions of type "pointer to FILE" that point to the FILE objects associated, respectively, with the standard error, input, and output streams:

extern FILE* stderr;
extern FILE* stdout;
extern FILE* stdin;

#define stderr stderr
//    Standard error output stream.
#define stdin stdin
//    Standard input stream.
#define stdout stdout
//    Standard output stream.

//The following shall be declared as functions and may also be defined as macros. Function prototypes shall be provided.

void     clearerr(FILE *);
//[CX][Option Start]
char    *ctermid(char *);
//[Option End]
int      fclose(FILE *);
//[CX][Option Start]
FILE    *fdopen(int, const char *);
//[Option End]
int      feof(FILE *);
int      ferror(FILE *);
int      fflush(FILE *);
int      fgetc(FILE *);
int      fgetpos(FILE *restrict, fpos_t *restrict);
char    *fgets(char *restrict, int, FILE *restrict);
//[CX][Option Start]
int      fileno(FILE *stream);
void     flockfile(FILE *);
FILE    *fmemopen(void *restrict, size_t, const char *restrict);
//[Option End]
FILE    *fopen(const char *restrict, const char *restrict);
int      fputc(int, FILE *);
int      fputs(const char *restrict, FILE *restrict);
size_t   fread(void *restrict, size_t, size_t, FILE *restrict);
FILE    *freopen(const char *restrict, const char *restrict,
             FILE *restrict);
int      fscanf(FILE *restrict, const char *restrict, ...);
int      fseek(FILE *, long, int);
//[CX][Option Start]
int      fseeko(FILE *, off_t, int);
//[Option End]
int      fsetpos(FILE *, const fpos_t *);
long     ftell(FILE *);
//[CX][Option Start]
off_t    ftello(FILE *);
int      ftrylockfile(FILE *);
void     funlockfile(FILE *);
//[Option End]
size_t   fwrite(const void *restrict, size_t, size_t, FILE *restrict);
int      getc(FILE *);
int      getchar(void);
//[CX][Option Start]
int      getc_unlocked(FILE *);
int      getchar_unlocked(void);
//TODO: This prototypes conflict with corutils

/* ssize_t  getdelim(char **, size_t *, int, */
/*              FILE *); */
/* ssize_t  getline(char **, size_t *, FILE *); */
int	 getdelim(char **, size_t *, int,
             FILE *);
int	 ft_getline(char **, size_t *, FILE *);

//[Option End]
//[OB][Option Start]
char    *gets(char *);
//[Option End]
//[CX][Option Start]
FILE    *open_memstream(char **, size_t *);
int      pclose(FILE *);
//[Option End]
void     perror(const char *);
//[CX][Option Start]
FILE    *popen(const char *, const char *);
//[Option End]
int      putc(int, FILE *);
int      putchar(int);
//[CX][Option Start]
int      putc_unlocked(int, FILE *);
int      putchar_unlocked(int);
//[Option End]
int      puts(const char *);
int      remove(const char *);
int      rename(const char *, const char *);
//[CX][Option Start]
int      renameat(int, const char *, int, const char *);
//[Option End]
void     rewind(FILE *);
int      scanf(const char *restrict, ...);
void     setbuf(FILE *restrict, char *restrict);
int      setvbuf(FILE *restrict, char *restrict, int, size_t);
int      sscanf(const char *restrict s, const char *restrict format, ...);
//[OB XSI][Option Start]
char    *tempnam(const char *, const char *);
//[Option End]
FILE    *tmpfile(void);
//[OB][Option Start]
char    *tmpnam(char *);
//[Option End]
int      ungetc(int c, FILE *stream);
int      vfscanf(FILE *restrict, const char *restrict, va_list);
int      vscanf(const char *restrict, va_list);
int      vsscanf(const char *restrict, const char *restrict, va_list);

//[CX] [Option Start] Inclusion of the <stdio.h> header may also make visible all symbols from <stddef.h>. [Option End]

/*
 * PRINTF FAMILY
 */

/*
 * #include <stdio.h>
 */
int printf(const char *format, ...);
int fprintf(FILE *stream, const char *format, ...);
int dprintf(int fd, const char *format, ...);

int sprintf(char *str, const char *format, ...);
int snprintf(char *str, size_t size, const char *format, ...);

int asprintf(char **strp, const char *format, ...);

/*
 * #include <stdarg.h>
 */
int vprintf(const char *format, va_list ap);
int vfprintf(FILE *stream, const char *format, va_list ap);
int vdprintf(int fd, const char *format, va_list ap);

int vsprintf(char *str, const char *format, va_list ap);
int vsnprintf(char *str, size_t size, const char *format, va_list ap);

int vasprintf(char **strp, const char *format, va_list ap);

/*
 * Custom printf methods
 */
int eprintf(const char *format, ...);

#endif
