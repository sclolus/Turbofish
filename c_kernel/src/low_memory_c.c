/*
 * c01020a6 <_init>:
 */
extern void _jump_init(int);

int tool(void);

#define LOCATE_FUNC  __attribute__((__section__(".low_memory.text")))
#define LOCATE_DATA  __attribute__((__section__(".low_memory.data")))

int LOCATE_DATA test = 42;

void LOCATE_FUNC _jump_c()
{
	_jump_init(test + tool());
}

void LOCATE_FUNC _jump_d()
{
	_jump_init(test);
}
