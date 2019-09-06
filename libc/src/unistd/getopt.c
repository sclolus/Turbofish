#include <unistd.h>
#include <string.h>
#include <stdint.h>
#include <stdlib.h>
#include <custom.h>

/* The getopt() function is a command-line parser that shall follow Utility Syntax Guidelines 3, 4, 5, 6, 7, 9, and 10 in XBD Utility Syntax Guidelines. */

/* The parameters argc and argv are the argument count and argument array as passed to main() (see exec()). The argument optstring is a string of recognized option characters; if a character is followed by a <colon>, the option takes an argument. All option characters allowed by Utility Syntax Guideline 3 are allowed in optstring. The implementation may accept other characters as an extension. */

/* The variable optind is the index of the next element of the argv[] vector to be processed. It shall be initialized to 1 by the system, and getopt() shall update it when it finishes with each element of argv[]. If the application sets optind to zero before calling getopt(), the behavior is unspecified. When an element of argv[] contains multiple option characters, it is unspecified how getopt() determines which options have already been processed. */

/* The getopt() function shall return the next option character (if one is found) from argv that matches a character in optstring, if there is one that matches. If the option takes an argument, getopt() shall set the variable optarg to point to the option-argument as follows: */

/* If the option was the last character in the string pointed to by an element of argv, then optarg shall contain the next element of argv, and optind shall be incremented by 2. If the resulting value of optind is greater than argc, this indicates a missing option-argument, and getopt() shall return an error indication. */

/* Otherwise, optarg shall point to the string following the option character in that element of argv, and optind shall be incremented by 1. */

/* If, when getopt() is called: */

/* argv[optind]  is a null pointer*argv[optind]  is not the character - */
/*  argv[optind]  points to the string "-" */

/* getopt() shall return -1 without changing optind. If: */

/* argv[optind]   points to the string "--" */

/* getopt() shall return -1 after incrementing optind. */

/* If getopt() encounters an option character that is not contained in optstring, it shall return the <question-mark> ( '?' ) character. If it detects a missing option-argument, it shall return the <colon> character ( ':' ) if the first character of optstring was a <colon>, or a <question-mark> character ( '?' ) otherwise. In either case, getopt() shall set the variable optopt to the option character that caused the error. If the application has not set the variable opterr to 0 and the first character of optstring is not a <colon>, getopt() shall also print a diagnostic message to stderr in the format specified for the getopts utility, unless the stderr stream has wide orientation, in which case the behavior is undefined. */

/* The getopt() function need not be thread-safe. */

#define GETOPT_STOP_OPTION "--"
#define GETOPT_ERR_CHAR '?'
#define GETOPT_ERR_ILLEGAL_OPTION 1
#define GETOPT_ERR_ILLEGAL_OPTION_MSG "illegal option -- "
#define GETOPT_ERR_NO_ARG 2
#define GETOPT_ERR_NO_ARG_MSG "option requires an argument -- "


char					*optarg;
int					optind = 1;
int					optopt;
int					opterr = 1;
int					optreset;

// We probably should remove this.
static void	error(uint32_t n, char **str) {
	uint32_t i = 0;

	while (i < n) {
		dprintf(2, "%s", str[i]);
		i++;
	}
	dprintf(2, "\n");
}

static int32_t	getopt_done(const int argc, char *const argv[], uint64_t *index)
{
	if (optreset)
	{
		*index = 1;
		optind = 1;
		optreset = 0;
	}
	if (optind >= argc
		|| !strcmp(argv[optind], GETOPT_STOP_OPTION)
		|| argv[optind][0] != '-')
		return (1);
	return (0);
}

static void		getopt_err(const char opt_char, const int error_type)
{
	if (!opterr)
		return ;
	if (error_type == GETOPT_ERR_ILLEGAL_OPTION)
		error(2, (char*[]){GETOPT_ERR_ILLEGAL_OPTION_MSG
					, (char[]){opt_char, '\0'}});
	else if (error_type == GETOPT_ERR_NO_ARG)
		error(2, (char*[]){GETOPT_ERR_NO_ARG_MSG
					, (char[]){opt_char, '\0'}});
	else
		error(1, (char*[]){"Unknown error"});
}

static int		getopt_argument(const int argc, char *const argv[]
								, uint64_t *index, const char *opt_char)
{
	if (!argv[optind][*index + 1])
	{
		if (optind + 1 >= argc)
		{
			getopt_err(*opt_char, GETOPT_ERR_NO_ARG);
			return ((int)GETOPT_ERR_CHAR);
		}
		*index = 1;
		optarg = argv[optind + 1];
		optind += 2;
		return ((int)*opt_char);
	}
	optarg = argv[optind] + *index + 1;
	*index = 1;
	optind++;
	return ((int)*opt_char);
}

int				getopt(int argc, char *const argv[], const char *optstring)
{
	static uint64_t		index = 1;
	char			*opt_char;

	if (getopt_done(argc, argv, &index))
		return (-1);
	if (argv[optind][index]
		&& (opt_char = strchr(optstring, argv[optind][index]))
		&& *opt_char != ':')
	{
		optopt = *opt_char;
		if (opt_char[1] == ':')
			return (getopt_argument(argc, argv, &index, (const char*)opt_char));
		index++;
		if (!argv[optind][index])
		{
			index = 1;
			optind++;
		}
		return (optopt);
	}
	getopt_err(argv[optind][index], GETOPT_ERR_ILLEGAL_OPTION);
	optopt = argv[optind][index++];
	return ((int)GETOPT_ERR_CHAR);
}

# warning missing tests
