OUTPUT_FORMAT("elf32-littlearm", "elf32-littlearm", "elf32-littlearm")
OUTPUT_ARCH(arm)

SECTIONS
{
  .text       : { 
    KEEP(*(.text.rustapi*)) /* 保留以 fontapi_ 开头的符号，也就是类似导出函数 */
    
    __text_start = . ;
    *(.init*)
    *(.text*)
    *(.ctors*)
    *(.dtors*)
    *(.rodata*)
    *(.fini*)
    *(.data*)
    *(COMMON*)
    __text_end  = . ;
    
    __bss_start__ = . ;
    *(.bss*)
    __bss_end__ = . ;
  }
  /* .data       : { *(.data*) } */
  /* .rodata     : { *(.rodata) } */
  /* .bss        : { *(.bss) } */
}
