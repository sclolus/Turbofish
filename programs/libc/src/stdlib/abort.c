#include <stdlib.h>
#include <signal.h>

void abort (void)
{
  while (1)
    {
      raise (SIGABRT);
      exit (1);
    }
}
