/*
long add(long a, long b) {
  return a+b;
}
*/
long add(long a, long b) {
    long c;
    asm volatile ("addl %%ebx,%%eax;"
                 : "=a" (c)              /* output: sum = EAX */
                 : "a" (a), "b" (b)  /* inputs: EAX = var1, EBX = var2 */
    );
    return c;
}
