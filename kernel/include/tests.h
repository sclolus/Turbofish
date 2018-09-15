
#ifndef __TESTS_H__
# define __TESTS_H__

enum mem_test_type {
	k_family = 0,
	v_family,
	k_sub_family
};

int	mem_test(enum mem_test_type type, int);
int	rand_test(void);

#endif
