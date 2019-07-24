#ifndef CTYPE_H
# define CTYPE_H
typedef int locale_t;
int   isalnum(int);
//[CX][Option Start]
int   isalnum_l(int, locale_t);
//[Option End]
int   isalpha(int);
//[CX][Option Start]
int   isalpha_l(int, locale_t);
//[Option End]
//[OB XSI][Option Start]
int   isascii(int);
//[Option End]
int   isblank(int);
//[CX][Option Start]
int   isblank_l(int, locale_t);
//[Option End]
int   iscntrl(int);
//[CX][Option Start]
int   iscntrl_l(int, locale_t);
//[Option End]
int   isdigit(int);
//[CX][Option Start]
int   isdigit_l(int, locale_t);
//[Option End]
int   isgraph(int);
//[CX][Option Start]
int   isgraph_l(int, locale_t);
//[Option End]
int   islower(int);
//[CX][Option Start]
int   islower_l(int, locale_t);
//[Option End]
int   isprint(int);
//[CX][Option Start]
int   isprint_l(int, locale_t);
//[Option End]
int   ispunct(int);
//[CX][Option Start]
int   ispunct_l(int, locale_t);
//[Option End]
int   isspace(int);
//[CX][Option Start]
int   isspace_l(int, locale_t);
//[Option End]
int   isupper(int);
//[CX][Option Start]
int   isupper_l(int, locale_t);
//[Option End]
int   isxdigit(int);
//[CX][Option Start]
int   isxdigit_l(int, locale_t);
//[Option End]
//[OB XSI][Option Start]
int   toascii(int);
//[Option End]
int   tolower(int);
//[CX][Option Start]
int   tolower_l(int, locale_t);
//[Option End]
int   toupper(int);
//[CX][Option Start]
int   toupper_l(int, locale_t);
//[Option End]

//The <ctype.h> header shall define the following as macros:

//[OB XSI][Option Start]
int   _toupper(int);
int   _tolower(int);
//[Option End]

#endif
 
