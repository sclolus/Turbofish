#ifndef __STDBOOL__H__
# define  __STDBOOL__H__

/* [CX] [Option Start] The functionality described on this reference page is aligned with the ISO C standard. Any conflict between the requirements described here and the ISO C standard is unintentional. This volume of POSIX.1-2017 defers to the ISO C standard. [Option End] */
/* The <stdbool.h> header shall define the following macros: */

/* bool */
/* Expands to _Bool. */
/* true */
/* Expands to the integer constant 1. */
/* false */
/* Expands to the integer constant 0. */
/* __bool_true_false_are_defined */
/* Expands to the integer constant 1. */
/* An application may undefine and then possibly redefine the macros bool, true, and false. */


#define bool _Bool
#define false 0
#define true 1
#define __bool_true_false_are_defined 1


#endif /* __STDBOOL__H__ */
