/* Memory layout of the LM3S6965 microcontroller */
/* 1K = 1 KiBi = 1024 bytes */
MEMORY
{
  FLASH : ORIGIN = 0x00000000, LENGTH = 256K
  RAM : ORIGIN = 0x20000000, LENGTH = 64K
}

/* The entry point is the reset handler */
ENTRY(Reset);

EXTERN(ISR_VECTORS);

SECTIONS
{
  .vector_table ORIGIN(FLASH) :
  {
    /* First entry: initial Stack Pointer value */
    LONG(ORIGIN(RAM) + LENGTH(RAM));

    /* Second entry: reset vector */
    KEEP(*(.vector_table.isr_vectors));
  } > FLASH

  .text :
  {
    *(.text .text.*);
  } > FLASH

  .data : 
  {
    _data = .;
    *(.data .data.*);
    _edata = .;
  } > RAM

  .bss :
  {
    _bss = .;
    *(.bss .bss.*);
    *(COMMON);
    _ebss = .;
  } > RAM
}
