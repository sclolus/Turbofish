#ifndef __LOCALE_H__
#define __LOCALE_H__


//The <locale.h> header shall define the lconv structure, which shall include at least the following members. (See the definitions of LC_MONETARY in LC_MONETARY and LC_NUMERIC.)
//
//char    *currency_symbol
//char    *decimal_point
//char     frac_digits
//char    *grouping
//char    *int_curr_symbol
//char     int_frac_digits
//char     int_n_cs_precedes
//char     int_n_sep_by_space
//char     int_n_sign_posn
//char     int_p_cs_precedes
//char     int_p_sep_by_space
//char     int_p_sign_posn
//char    *mon_decimal_point
//char    *mon_grouping
//char    *mon_thousands_sep
//char    *negative_sign
//char     n_cs_precedes
//char     n_sep_by_space
//char     n_sign_posn
//char    *positive_sign
//char     p_cs_precedes
//char     p_sep_by_space
//char     p_sign_posn
//char    *thousands_sep
//
//The <locale.h> header shall define NULL (as described in <stddef.h>) and at least the following as macros:
//
//
//LC_ALL
//LC_COLLATE
//LC_CTYPE
//[CX] [Option Start]
//LC_MESSAGES
//[Option End]
//LC_MONETARY
//LC_NUMERIC
//LC_TIME
//
//which shall expand to integer constant expressions with distinct values for use as the first argument to the setlocale() function.
//
//Additional macro definitions, beginning with the characters LC_ and an uppercase letter, may also be specified by the implementation.
//
//[CX] [Option Start] The <locale.h> header shall contain at least the following macros representing bitmasks for use with the newlocale() function for each supported locale category: LC_COLLATE_MASK LC_CTYPE_MASK LC_MESSAGES_MASK LC_MONETARY_MASK LC_NUMERIC_MASK LC_TIME_MASK
//
//In addition, a macro to set the bits for all categories set shall be defined: LC_ALL_MASK
//
//The <locale.h> header shall define LC_GLOBAL_LOCALE, a special locale object descriptor used by the duplocale() and uselocale() functions.
//
//The <locale.h> header shall define the locale_t type, representing a locale object. [Option End]

typedef int locale_t;
//
//The following shall be declared as functions and may also be defined as macros. Function prototypes shall be provided for use with ISO C standard compilers.
//
//[CX][Option Start]
//locale_t      duplocale(locale_t);
//void          freelocale(locale_t);
//[Option End]
//struct lconv *localeconv(void);
//[CX][Option Start]
//locale_t      newlocale(int, const char *, locale_t);
//[Option End]
//char         *setlocale(int, const char *);
//[CX][Option Start]
//locale_t      uselocale (locale_t);
//[Option End]


# endif
