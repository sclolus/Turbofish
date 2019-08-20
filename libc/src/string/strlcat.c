#include <string.h>

size_t	strlcat(char *restrict dst, const char *restrict src, size_t size)
{
	size_t src_len;
	size_t dst_len;

	src_len = 0;
	while (src[src_len])
		src_len++;
	dst_len = 0;
	while (*dst++)
		dst_len++;
	dst -= 1;
	if (dst_len >= size)
		return (size + src_len);
	size -= dst_len + 1;
	while (*src && size--)
		*dst++ = *src++;
	*dst = '\0';
	return (src_len + dst_len);
}
