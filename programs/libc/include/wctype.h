#ifndef __WCTYPE_H__
# define __WCTYPE_H__
//[CX] [Option Start] Some of the functionality described on this reference page extends the ISO C standard. Applications shall define the appropriate feature test macro (see XSH The Compilation Environment ) to enable the visibility of these symbols in this header. [Option End]

//The <wctype.h> header shall define the following types:

#include <wchar.h>
#include <locale.h>
//wint_t
//    As described in <wchar.h>.
#define wctrans_t int
//    A scalar type that can hold values which represent locale-specific character mappings.
//wctype_t
//    As described in <wchar.h>.

//[CX] [Option Start] The <wctype.h> header shall define the locale_t type as described in <locale.h>. [Option End]
//
//The <wctype.h> header shall define the following macro:
//
//WEOF
//    As described in <wchar.h>.
//
//For all functions described in this header that accept an argument of type wint_t, the value is representable as a wchar_t or equals the value of WEOF. If this argument has any other value, the behavior is undefined.
//
//The behavior of these functions shall be affected by the LC_CTYPE category of the current locale.
//
//[CX] [Option Start] Inclusion of the <wctype.h> header may make visible all symbols from the headers <ctype.h>, <stdarg.h>, <stddef.h>, <stdio.h>, <stdlib.h>, <string.h>, <time.h>, and <wchar.h>. [Option End]
//
//The following shall be declared as functions and may also be defined as macros. Function prototypes shall be provided for use with ISO C standard compilers.

int       iswalnum(wint_t);
//[CX][Option Start]
int       iswalnum_l(wint_t, locale_t);
//[Option End]
int       iswalpha(wint_t);
//[CX][Option Start]
int       iswalpha_l(wint_t, locale_t);
//[Option End]
int       iswblank(wint_t);
//[CX][Option Start]
int       iswblank_l(wint_t, locale_t);
//[Option End]
int       iswcntrl(wint_t);
//[CX][Option Start]
int       iswcntrl_l(wint_t, locale_t);
//[Option End]
int       iswctype(wint_t, wctype_t);
//[CX][Option Start]
int       iswctype_l(wint_t, wctype_t, locale_t);
//[Option End]
int       iswdigit(wint_t);
//[CX][Option Start]
int       iswdigit_l(wint_t, locale_t);
//[Option End]
int       iswgraph(wint_t);
//[CX][Option Start]
int       iswgraph_l(wint_t, locale_t);
//[Option End]
int       iswlower(wint_t);
//[CX][Option Start]
int       iswlower_l(wint_t, locale_t);
//[Option End]
int       iswprint(wint_t);
//[CX][Option Start]
int       iswprint_l(wint_t, locale_t);
//[Option End]
int       iswpunct(wint_t);
//[CX][Option Start]
int       iswpunct_l(wint_t, locale_t);
//[Option End]
int       iswspace(wint_t);
//[CX][Option Start]
int       iswspace_l(wint_t, locale_t);
//[Option End]
int       iswupper(wint_t);
//[CX][Option Start]
int       iswupper_l(wint_t, locale_t);
//[Option End]
int       iswxdigit(wint_t);
//[CX][Option Start]
int       iswxdigit_l(wint_t, locale_t);
//[Option End]
wint_t    towctrans(wint_t, wctrans_t);
//[CX][Option Start]
wint_t    towctrans_l(wint_t, wctrans_t, locale_t);
//[Option End]
wint_t    towlower(wint_t);
//[CX][Option Start]
wint_t    towlower_l(wint_t, locale_t);
//[Option End]
wint_t    towupper(wint_t);
//[CX][Option Start]
wint_t    towupper_l(wint_t, locale_t);
//[Option End]
wctrans_t wctrans(const char *);
//[CX][Option Start]
wctrans_t wctrans_l(const char *, locale_t);
//[Option End]
wctype_t  wctype(const char *);
//[CX][Option Start]
wctype_t  wctype_l(const char *, locale_t);
//[Option End]


#endif
