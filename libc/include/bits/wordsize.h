#ifndef __WORDSIZE_H__
# define __WORDSIZE_H__
/* Determine the wordsize from the preprocessor defines.  */

#if defined __x86_64__
# define __WORDSIZE	64
#elif __i386__
# define __WORDSIZE	32
#elif
# error Could not determine the word-size of the system.
#endif

#endif /* __WORDSIZE_H__ */
