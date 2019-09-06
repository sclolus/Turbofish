#include <stdio.h>
#include <custom.h>

#warning DUMMY IMPLEMENTATION

/// The getc() function shall be equivalent to fgetc, except that if it is implemented as a macro it may evaluate stream more than once, so the argument should never be an expression with side-effects.
# warning "thread-safety for getc hasn't been implemented yet."
int getc(FILE *stream)
{
	return fgetc(stream);
}


# warning DUMMY IMPLEMENTATION
int getc_unlocked(FILE *stream) {
	return getc(stream);
}
