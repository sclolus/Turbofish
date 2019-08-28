#include <unistd.h>

int main() {
	if (!isatty(0)) {
		return 1;
	}
	if (isatty(42) == 1) {
		return 1;
	}
	return 0;
}
