
#include "i386_type.h"
#include "libft.h"

extern int get_cpu_features(void);

int debug_center(void) {
	printk("Les carotes sont cuites\n");
	printk("{red}Ce sont des choses qui arrivent{eoc}\n");
	printk("Les carotes sont cuites\n");
	printk("{red}Ce sont des choses qui arrivent{eoc}\n");
	printk("Les carotes sont cuites\n");
	printk("{red}Ce sont des choses qui arrivent{eoc}\n");
	printk("Les carotes sont cuites\n");
	printk("Ce sont des choses qui arrivent");
	while(1);
	return get_cpu_features();
}
