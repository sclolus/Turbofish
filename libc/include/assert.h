#ifndef __ASSERT_H__
# define __ASSERT_H__

# include <stdio.h> //eeeeeeh.
# include <stdlib.h>
__attribute__((noreturn, always_inline)) inline static void	__assertion_failure(const char *assertion, const char *file, unsigned int line)
{
	dprintf(2, "assertion failed(%s:%u): %s\n", file, line, assertion);
	abort();
}

# define assert(assertion) ((assertion) ? (void)0 : __assertion_failure(#assertion, __FILE__, __LINE__))

#endif  /* __ASSERT_H__ */
