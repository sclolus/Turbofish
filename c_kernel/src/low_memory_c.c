extern void _init(void);

#define LOCATE_FUNC  __attribute__((__section__(".low_memory")))

void LOCATE_FUNC _jump_c(void)
{
	_init();
}
