#include <setjmp.h>

int sigsetjmp(sigjmp_buf env, int savesigs)
{
	(void)env;
	(void)savesigs;
	abort();
}

void siglongjmp(sigjmp_buf env, int val)
{
	(void)env;
	(void)val;
	abort();
}
