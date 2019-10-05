#include <assert.h>
#include <stdlib.h>
#include <stdbool.h>

// Should be initialized to false anyway...
static bool was_constructed = false;

__attribute__((constructor)) void	constructor(void)
{
	was_constructed = true;
}

int main(void)
{
	assert(was_constructed);
	return EXIT_SUCCESS;
}
