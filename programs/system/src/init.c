#include <sys/wait.h>

int main() {
	while (1) {
		int status;
		wait(&status);
	}
}
