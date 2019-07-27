#ifndef __NL_TYPES_H__
# define __NL_TYPES_H__

//The <nl_types.h> header shall define at least the following types:

typedef int nl_catd;
//    Used by the message catalog functions catopen(), catgets(), and catclose() to identify a catalog descriptor.
typedef int nl_item;
//    Used by nl_langinfo() to identify items of langinfo data. Values of objects of type nl_item are defined in <langinfo.h>.

//The <nl_types.h> header shall define at least the following symbolic constants:

/* #define NL_SETD 42 */
//    Used by gencat when no $set directive is specified in a message text source file. This constant can be passed as the value of set_id on subsequent calls to catgets() (that is, to retrieve messages from the default message set). The value of NL_SETD is implementation-defined.
/* NL_CAT_LOCALE */
//    Value that must be passed as the oflag argument to catopen() to ensure that message catalog selection depends on the LC_MESSAGES locale category, rather than directly on the LANG environment variable.

//The following shall be declared as functions and may also be defined as macros. Function prototypes shall be provided.

int       catclose(nl_catd);
char     *catgets(nl_catd, int, int, const char *);
nl_catd   catopen(const char *, int);

#endif
