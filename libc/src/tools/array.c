
#include <stdlib.h>
#include <tools.h>

uint32_t array_size(void **array) {
	uint32_t i = 0;

	while (array[i]) {
		i++;
	}
	return i;
}

void	free_array(void **array) {
	uint32_t i = 0;

	while (array[i]) {
		free(array[i]);
		i++;
	}
	free(array);
}
