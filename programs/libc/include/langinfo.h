#ifndef __LANGINFO_H__
# define __LANGINFO_H__

#include <locale.h>
#include <nl_types.h>

//The <langinfo.h> header shall define the symbolic constants used to identify items of langinfo data (see nl_langinfo()).
//The <langinfo.h> header shall define the locale_t type as described in <locale.h>.
//
//The <langinfo.h> header shall define the nl_item type as described in <nl_types.h>.
//
//The <langinfo.h> header shall define the following symbolic constants with type nl_item. The entries under Category indicate in which setlocale() category each item is defined.

//If the locale's values for p_cs_precedes and n_cs_precedes do not match, the value of nl_langinfo(CRNCYSTR) and nl_langinfo_l(CRNCYSTR,loc) is unspecified.
//
//The following shall be declared as a function and may also be defined as a macro. A function prototype shall be provided.
//

char *nl_langinfo(nl_item);
char *nl_langinfo_l(nl_item, locale_t);
//
//Inclusion of the <langinfo.h> header may also make visible all symbols from <nl_types.h>.

//TODO: check that not posix
#define CODESET			42

#endif
