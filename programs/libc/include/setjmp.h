#ifndef __SETJMP_H__
# define __SETJMP_H__
//[CX] [Option Start] Some of the functionality described on this reference page extends the ISO C standard. Applications shall define the appropriate feature test macro (see XSH The Compilation Environment ) to enable the visibility of these symbols in this header. [Option End]

typedef int jmp_buf[24];
typedef int sigjmp_buf[24];
//The <setjmp.h> header shall define the array types jmp_buf and [CX] [Option Start] sigjmp_buf. [Option End]

//The following shall be declared as functions and may also be defined as macros. Function prototypes shall be provided.

//[OB XSI][Option Start]
void   _longjmp(jmp_buf, int);
//[Option End]
void   longjmp(jmp_buf, int);
//[CX][Option Start]
void   siglongjmp(sigjmp_buf, int);
//[Option End]

//The following may be declared as functions, or defined as macros, or both. If functions are declared, function prototypes shall be provided.

//[OB XSI][Option Start]
int    _setjmp(jmp_buf);
//[Option End]
int    setjmp(jmp_buf);
//[CX][Option Start]
int    sigsetjmp(sigjmp_buf, int);
//[Option End]


#endif
