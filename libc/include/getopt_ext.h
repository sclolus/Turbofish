#ifndef __GETOPT_EXT_H__
# define __GETOPT_EXT_H__

struct option
{
  const char *name;
  /* has_arg can't be an enum because some compilers complain about
     type mismatches in all the code that assumes it is an int.  */
  int has_arg;
  int *flag;
  int val;
};

/* Names for the values of the 'has_arg' field of 'struct option'.  */

#define no_argument		0
#define required_argument	1
#define optional_argument	2

int getopt_long(int argc, char **argv,
			const char *shortopts,
		const struct option *longopts, int *longind);



#endif /* __GETOPT_EXT_H__ */
