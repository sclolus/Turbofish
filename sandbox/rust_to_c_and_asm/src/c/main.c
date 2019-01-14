#include <stdlib.h>
#include <stdio.h>
extern int rust_jump(int i);

int main(void) {
	rust_jump(42);

//	exit(1);
	return 0;
}

struct mastruc {
	int a;
	int b;
};

int	c_jump(int a, int *ptr, struct mastruc m) {
	printf("first param int : '%d'\nsecond param ptr:  '%p' , '%d'\nthird param struct:'%d', '%d'\n",a, ptr, *ptr, m.a, m.b);  
	return 3;
}
