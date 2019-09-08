#include <ltrace.h>
#include <errno.h>
#include <stdlib.h>
#include <stdint.h>

extern char **environ;
///       The  clearenv()  function clears the environment of all name-value pairs and sets the
///       value of the external variable environ to NULL.  After this call, new  variables  can
///       be added to the environment using putenv(3) and setenv(3).


static void free_array(void **array) {
	for (uint32_t i = 0; array[i]; i++) {
		free(array[i]);
	}
	free(array);
}

/// We might want not to free the strings inside the environ...
int clearenv(void) {
	if (environ) {
		free_array((void**)environ);
		environ = NULL;
	}
	return 0;
}
