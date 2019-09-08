#include <ltrace.h>
#include <ctype.h>
#include <stdio.h>
#include <stdarg.h>
#include <string.h>
#include <stdlib.h>

#define COPY_VA_INT \
  do { \
	 const int value = abs (va_arg (ap, int)); \
	 char buf[32]; \
	 ptr++; /* Go past the asterisk.  */ \
	 *sptr = '\0'; /* NULL terminate sptr.  */ \
	 sprintf(buf, "%d", value); \
	 strcat(sptr, buf); \
	 while (*sptr) sptr++; \
     } while (0)

#define PRINT_CHAR(CHAR) \
  do { \
	 putc(CHAR, stream); \
	 ptr++; \
	 total_printed++; \
	 continue; \
     } while (0)

#define PRINT_TYPE(TYPE) \
  do { \
	int result; \
	TYPE value = va_arg (ap, TYPE); \
	*sptr++ = *ptr++; /* Copy the type specifier.  */ \
	*sptr = '\0'; /* NULL terminate sptr.  */ \
	result = fprintf(stream, specifier, value); \
	if (result == -1) \
	  return -1; \
	else \
	  { \
	    total_printed += result; \
	    continue; \
	  } \
      } while (0)

/*
 * OBSOLETE: Equivalent to vfprintf(FILE *stream, const char *format, va_list ap);
 */
int
_doprnt(const char *format, va_list ap, FILE *stream)
{
	TRACE
  const char * ptr = format;
  char specifier[128];
  int total_printed = 0;
  
  while (*ptr != '\0')
    {
      if (*ptr != '%') /* While we have regular characters, print them.  */
	PRINT_CHAR(*ptr);
      else /* We got a format specifier! */
	{
	  char * sptr = specifier;
	  int wide_width = 0, short_width = 0;
	  
	  *sptr++ = *ptr++; /* Copy the % and move forward.  */

	  while (strchr ("-+ #0", *ptr)) /* Move past flags.  */
	    *sptr++ = *ptr++;

	  if (*ptr == '*')
	    COPY_VA_INT;
	  else
	    while (isdigit(*ptr)) /* Handle explicit numeric value.  */
	      *sptr++ = *ptr++;
	  
	  if (*ptr == '.')
	    {
	      *sptr++ = *ptr++; /* Copy and go past the period.  */
	      if (*ptr == '*')
		COPY_VA_INT;
	      else
		while (isdigit(*ptr)) /* Handle explicit numeric value.  */
		  *sptr++ = *ptr++;
	    }
	  while (strchr ("hlL", *ptr))
	    {
	      switch (*ptr)
		{
		case 'h':
		  short_width = 1;
		  break;
		case 'l':
		  wide_width++;
		  break;
		case 'L':
		  wide_width = 2;
		  break;
		default:
		  abort();
		}
	      *sptr++ = *ptr++;
	    }

	  switch (*ptr)
	    {
	    case 'd':
	    case 'i':
	    case 'o':
	    case 'u':
	    case 'x':
	    case 'X':
	    case 'c':
	      {
		/* Short values are promoted to int, so just copy it
                   as an int and trust the C library printf to cast it
                   to the right width.  */
		if (short_width)
		  PRINT_TYPE(int);
		else
		  {
		    switch (wide_width)
		      {
		      case 0:
			PRINT_TYPE(int);
			break;
		      case 1:
			PRINT_TYPE(long);
			break;
		      case 2:
		      default:
#if defined(__GNUC__) || defined(HAVE_LONG_LONG)
			PRINT_TYPE(long long);
#else
			PRINT_TYPE(long); /* Fake it and hope for the best.  */
#endif
			break;
		      } /* End of switch (wide_width) */
		  } /* End of else statement */
	      } /* End of integer case */
	      break;
	    case 'f':
	    case 'e':
	    case 'E':
	    case 'g':
	    case 'G':
	      {
		if (wide_width == 0)
		  PRINT_TYPE(double);
		else
		  {
#if defined(__GNUC__) || defined(HAVE_LONG_DOUBLE)
		    PRINT_TYPE(long double);
#else
		    PRINT_TYPE(double); /* Fake it and hope for the best.  */
#endif
		  }
	      }
	      break;
	    case 's':
	      PRINT_TYPE(char *);
	      break;
	    case 'p':
	      PRINT_TYPE(void *);
	      break;
	    case '%':
	      PRINT_CHAR('%');
	      break;
	    default:
	      abort();
	    } /* End of switch (*ptr) */
	} /* End of else statement */
    }

  return total_printed;
}
