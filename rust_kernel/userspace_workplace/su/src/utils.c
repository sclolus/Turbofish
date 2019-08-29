#include "su.h"

uint32_t array_size(char **array) {
	uint32_t i = 0;

	while (array[i]) {
		i++;
	}
	return i;
}

void	free_array(char **array) {
	uint32_t i = 0;

	while (array[i]) {
		free(array[i]);
		i++;
	}
	free(array);
}
