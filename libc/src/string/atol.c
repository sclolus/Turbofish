#include <ltrace.h>
#include <string.h>
#include <stdlib.h>

long	atol(const char *str)
{
	TRACE
	long result;
	long sign;

	result = 0;
	sign = 1;
	while ((*str >= 9 && *str <= 13) || *str == 32)
		str++;
	if (*str == '-' || *str == '+') {
		if (*str == '-')
			sign *= -1;
		str++;
	}
	while (*str >= '0' && *str <= '9') {
		result = result * 10 + (*str - '0');
		str++;
	}
	return (sign * result);
}
