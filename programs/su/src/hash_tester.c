#include "su.h"
#include <stddef.h>

#ifdef TESTS
int			hash_tester(void *message
						, uint32_t *to_test_digest
						, uint64_t len
						, t_hash_info *hash_info)
{
	uint32_t	*diff;
	static char	salt[256];

	memset(salt, 0, sizeof(salt));
	strcat(salt, "$1$");
	strcat(salt, hash_info->salt);
	strcat(salt, "$");

	/* assert(sizeof(diff) >= hash_info->digest_size); */
	diff = hash_info->system_hash(message, salt);
	if (memcmp(to_test_digest, diff, hash_info->digest_size))
	{
		printf("------>digest memory\n");
		print_memory(to_test_digest, hash_info->digest_size);
		printf("------>true digest memory\n");
		print_memory(diff, hash_info->digest_size);
		/* printf("\noriginal string: \"%s\"\n", message); */
		printf("my_hash:  ");
		print_hash(to_test_digest, hash_info->digest_size, 1);
		printf("\n");
		printf("true_hash:  ");
		print_hash(diff, hash_info->digest_size, 1);
		printf("\n");
		printf("FAILURE\n");
		return (0);
	}
	return (1);
}
#endif
