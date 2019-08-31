# include "su.h"

#ifndef _GETOPT_H_
# define _GETOPT_H_
#  define GETOPT_STOP_OPTION "--"
#  define GETOPT_ERR_CHAR '?'
#  define GETOPT_ERR_ILLEGAL_OPTION 1
#  define GETOPT_ERR_ILLEGAL_OPTION_MSG "illegal option -- "
#  define GETOPT_ERR_NO_ARG 2
#  define GETOPT_ERR_NO_ARG_MSG "option requires an argument -- "

extern char	*g_optarg;
extern int	g_optind;
extern int	g_optopt;
extern int	g_opterr;
extern int	g_optreset;

int	_getopt(int argc, char *const argv[], const char *optstring);

#endif /* _GETOPT_H_ */
