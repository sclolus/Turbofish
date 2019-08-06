#ifndef __INTTYPES_H__
# define __INTTYPES_H__
//    [CX] [Option Start] Some of the functionality described on this reference page extends the ISO C standard. Applications shall define the appropriate feature test macro (see XSH The Compilation Environment ) to enable the visibility of these symbols in this header. [Option End]

#include <i386.h>
#include <stdint.h>
#include <stddef.h>
//The <inttypes.h> header shall include the <stdint.h> header.

//The <inttypes.h> header shall define at least the following types:

typedef struct imaxdiv {
	int quot;
	int rem;
} imaxdiv_t;
//    Structure type that is the type of the value returned by the imaxdiv() function.
//wchar_t
//    [CX] [Option Start] As described in <stddef.h>. [Option End]

//The <inttypes.h> header shall define the following macros. Each expands to a character string literal containing a conversion specifier, possibly modified by a length modifier, suitable for use within the format argument of a formatted input/output function when converting the corresponding integer type. These macros have the general form of PRI (character string literals for the fprintf() and fwprintf() family of functions) or SCN (character string literals for the fscanf() and fwscanf() family of functions), followed by the conversion specifier, followed by a name corresponding to a similar type name in <stdint.h>. In these names, N represents the width of the type as described in <stdint.h>. For example, PRIdFAST32 can be used in a format string to print the value of an integer of type int_fast32_t.

//The fprintf() macros for signed integers are:

//    PRIdN
//    	
//
//    PRIdLEASTN
//    	
//
//    PRIdFASTN
//    	
//
//    PRIdMAX
//    	
//
//    PRIdPTR
//
//    PRIiN
//    	
//
//    PRIiLEASTN
//    	
//
//    PRIiFASTN
//    	
//
//    PRIiMAX
//    	
//
//    PRIiPTR
//
//The fprintf() macros for unsigned integers are:
//
//    PRIoN
//    	
//
//    PRIoLEASTN
//    	
//
//    PRIoFASTN
//    	
//
//    PRIoMAX
//    	
//
//    PRIoPTR
//
//    PRIuN
//    	
//
//    PRIuLEASTN
//    	
//
//    PRIuFASTN
//    	
//
//    PRIuMAX
//    	
//
//    PRIuPTR
//
//    PRIxN
//    	
//
//    PRIxLEASTN
//    	
//
//    PRIxFASTN
//    	
//
//    PRIxMAX
//    	
//
//    PRIxPTR
//
//    PRIXN
//    	
//
//    PRIXLEASTN
//    	
//
//    PRIXFASTN
//    	
//
//    PRIXMAX
//    	
//
//    PRIXPTR
//
//The fscanf() macros for signed integers are:
//
//    SCNdN
//    	
//
//    SCNdLEASTN
//    	
//
//    SCNdFASTN
//    	
//
//    SCNdMAX
//    	
//
//    SCNdPTR
//
//    SCNiN
//    	
//
//    SCNiLEASTN
//    	
//
//    SCNiFASTN
//    	
//
//    SCNiMAX
//    	
//
//    SCNiPTR
//
//The fscanf() macros for unsigned integers are:
//
//    SCNoN
//    	
//
//    SCNoLEASTN
//    	
//
//    SCNoFASTN
//    	
//
//    SCNoMAX
//    	
//
//    SCNoPTR
//
//    SCNuN
//    	
//
//    SCNuLEASTN
//    	
//
//    SCNuFASTN
//    	
//
//    SCNuMAX
//    	
//
//    SCNuPTR
//
//    SCNxN
//    	
//
//    SCNxLEASTN
//    	
//
//    SCNxFASTN
//    	
//
//    SCNxMAX
//    	
//
//    SCNxPTR

//For each type that the implementation provides in <stdint.h>, the corresponding fprintf() and fwprintf() macros shall be defined and the corresponding fscanf() and fwscanf() macros shall be defined unless the implementation does not have a suitable modifier for the type.

//The following shall be declared as functions and may also be defined as macros. Function prototypes shall be provided.

intmax_t imaxabs(intmax_t);
imaxdiv_t imaxdiv(intmax_t, intmax_t);
intmax_t strtoimax(const char *restrict, char **restrict, int);
uintmax_t strtoumax(const char *restrict, char **restrict, int);
intmax_t wcstoimax(const wchar_t *restrict, wchar_t **restrict, int);
uintmax_t wcstoumax(const wchar_t *restrict, wchar_t **restrict, int);

#endif
