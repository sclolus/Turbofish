#ifndef _STDLIB_H
# define _STDLIB_H

#ifdef __cplusplus
extern "C" {
#endif
//The <stdlib.h> header shall define the following macros which shall expand to integer constant expressions:

#define EXIT_FAILURE 1
//    Unsuccessful termination for exit(); evaluates to a non-zero value.
#define EXIT_SUCCESS 0
//    Successful termination for exit(); evaluates to 0.
#define RAND_MAX 32767
//    Maximum value returned by rand(); at least 32767.

//The <stdlib.h> header shall define the following macro which shall expand to a positive integer expression with type size_t:

#define MB_CUR_MAX 4
//    Maximum number of bytes in a character specified by the current locale (category LC_CTYPE).

////[CX] [Option Start] In the POSIX locale the value of {MB_CUR_MAX} shall be 1. [Option End]

//The <stdlib.h> header shall define NULL as described in <stddef.h>.
#include <stddef.h>

//The <stdlib.h> header shall define the following data types through typedef:

typedef int div_t;
//    Structure type returned by the div() function.
typedef int ldiv_t;
//    Structure type returned by the ldiv() function.
typedef int lldiv_t;
//   Structure type returned by the lldiv() function.
//typedef size_t int;
//    As described in <stddef.h>.
//typedef wchar_t int;
//    As described in <stddef.h>.

////[CX] [Option Start] In addition, the <stdlib.h> header shall define the following symbolic constants and macros as described in <sys/wait.h>:


//WEXITSTATUS
//WIFEXITED
//WIFSIGNALED
//WIFSTOPPED
//WNOHANG
//WSTOPSIG
//WTERMSIG
//WUNTRACED
////[Option End]

//The following shall be declared as functions and may also be defined as macros. Function prototypes shall be provided.

void          _Exit(int);
////[XSI][Option Start]
long          a64l(const char *);
////[Option End]
void          abort(void);
int           abs(int);
int           atexit(void (*)(void));
double        atof(const char *);
int           atoi(const char *);
long          atol(const char *);
long long     atoll(const char *);
void         *bsearch(const void *, const void *, size_t, size_t,
                  int (*)(const void *, const void *));
void         *calloc(size_t, size_t);
div_t         div(int, int);
//[XSI][Option Start]
double        drand48(void);
double        erand48(unsigned short [3]);
//[Option End]
void          exit(int);
void          free(void *);
char         *getenv(const char *);
//[CX][Option Start]
int           getsubopt(char **, char *const *, char **);
//[Option End]
//[XSI][Option Start]
int           grantpt(int);
char         *initstate(unsigned, char *, size_t);
long          jrand48(unsigned short [3]);
char         *l64a(long);
//[Option End]
long          labs(long);
//[XSI][Option Start]
void          lcong48(unsigned short [7]);
//[Option End]
ldiv_t        ldiv(long, long);
long long     llabs(long long);
lldiv_t       lldiv(long long, long long);
//[XSI][Option Start]
long          lrand48(void);
//[Option End]
void         *malloc(size_t);
int           mblen(const char *, size_t);
size_t        mbstowcs(wchar_t *restrict, const char *restrict, size_t);
int           mbtowc(wchar_t *restrict, const char *restrict, size_t);
//[CX][Option Start]
char         *mkdtemp(char *);
int           mkstemp(char *);
//[Option End]
//[XSI][Option Start]
long          mrand48(void);
long          nrand48(unsigned short [3]);
//[Option End]
//[ADV][Option Start]
int           posix_memalign(void **, size_t, size_t);
//[Option End]
//[XSI][Option Start]
int           posix_openpt(int);
char         *ptsname(int);
int           putenv(char *);
//[Option End]
void          qsort(void *, size_t, size_t, int (*)(const void *,
                  const void *));
int           rand(void);
//[OB CX][Option Start]
int           rand_r(unsigned *);
//[Option End]
//[XSI][Option Start]
long          random(void);
//[Option End]
void         *realloc(void *, size_t);
//[XSI][Option Start]
char         *realpath(const char *restrict, char *restrict);
unsigned short *seed48(unsigned short[3]);
//[Option End]
//[CX][Option Start]
int           setenv(const char *, const char *, int);
//[Option End]
//[XSI][Option Start]
void          setkey(const char *);
char         *setstate(char *);
//[Option End]
void          srand(unsigned);
//[XSI][Option Start]
void          srand48(long);
void          srandom(unsigned);
//[Option End]
double        strtod(const char *restrict, char **restrict);
float         strtof(const char *restrict, char **restrict);
long          strtol(const char *restrict, char **restrict, int);
long double   strtold(const char *restrict, char **restrict);
long long     strtoll(const char *restrict, char **restrict, int);
unsigned long strtoul(const char *restrict, char **restrict, int);
unsigned long long
              strtoull(const char *restrict, char **restrict, int);
int           system(const char *);
//[XSI][Option Start]
int           unlockpt(int);
//[Option End]
//[CX][Option Start]
int           unsetenv(const char *);
//[Option End]
size_t        wcstombs(char *restrict, const wchar_t *restrict, size_t);
int           wctomb(char *, wchar_t);


#ifdef __cplusplus
}
#endif

#endif
