#include <string.h>
#include <stdio.h>

void	putnbr_base(int n, int base)
{
	int	exponent;
	int	sign;
	int	i;
	char	buff;

	sign = (n < 0) ? 1 : 0;
	exponent = 1;
	i = n;
	while ((i = i / base))
		exponent *= base;
	if (sign)
		putchar('-');
	while (exponent) {
		i = n / exponent;
		buff = (sign) ? HEX_T(-(i % base)) : HEX_T((i % base));
		n -= i * exponent;
		putchar(buff);
		exponent /= base;
	}
}
