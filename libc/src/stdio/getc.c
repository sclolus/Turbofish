#include <stdio.h>
#include <unistd.h>
#include <stdbool.h>
#include <custom.h>

/// If the end-of-file indicator for the input stream pointed to by
/// stream is not set and a next byte is present, the fgetc() function
/// shall obtain the next byte as an unsigned char converted to an
/// int, from the input stream pointed to by stream, and advance the
/// associated file position indicator for the stream (if
/// defined). Since fgetc() operates on bytes, reading a character
/// consisting of multiple bytes (or "a multi-byte character") may
/// require multiple calls to fgetc().
///
/// [CX] [Option Start] The fgetc() function may mark the last data
/// access timestamp of the file associated with stream for
/// update. The last data access timestamp shall be marked for update
/// by the first successful execution of fgetc(), fgets(), fread(),
/// fscanf(), getc(), getchar(), getdelim(), getline(), gets(), or
/// scanf() using stream that returns data not supplied by a prior
/// call to ungetc(). [Option End]
/// Upon successful completion, fgetc() shall return the next byte
/// from the input stream pointed to by stream. If the end-of-file
/// indicator for the stream is set, or if the stream is at
/// end-of-file, the end-of-file indicator for the stream shall be set
/// and fgetc() shall return EOF. If a read error occurs, the error
/// indicator for the stream shall be set, fgetc() shall return EOF,
/// [CX] [Option Start] and shall set errno to indicate the
/// error. [Option End]


/// If the end-of-file indicator for the input stream pointed to by stream is not set and a next byte is present, the fgetc() function shall obtain the next byte as an unsigned char converted to an int, from the input stream pointed to by stream, and advance the associated file position indicator for the stream (if defined). Since fgetc() operates on bytes, reading a character consisting of multiple bytes (or "a multi-byte character") may require multiple calls to fgetc().

/// RETURN VALUE
/// Upon successful completion, fgetc() shall return the next byte from the input stream pointed to by stream. If the end-of-file indicator for the stream is set, or if the stream is at end-of-file, the end-of-file indicator for the stream shall be set and fgetc() shall return EOF. If a read error occurs, the error indicator for the stream shall be set, fgetc() shall return EOF, [CX] [Option Start]  and shall set errno to indicate the error. [Option End]

/// The getc() function shall be equivalent to fgetc, except that if it is implemented as a macro it may evaluate stream more than once, so the argument should never be an expression with side-effects.
# warning "thread-safety for getc hasn't been implemented yet."
int getc(FILE *stream)
{
	return fgetc(stream);
}
