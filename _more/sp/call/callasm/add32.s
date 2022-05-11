.text
.globl _my_adder
_my_adder:
        addl %edi, %esi
        movl %esi, %eax
        ret
