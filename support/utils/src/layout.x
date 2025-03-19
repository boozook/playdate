/* Elf layout that acceptable for PDC */

ENTRY(eventHandlerShim)

MEMORY
{
	/*
		Stack: 61800
		Heap: 8388208
	*/
	ram (rwx) : ORIGIN = 0, LENGTH = 8388208
}
PROVIDE(_fstack = ORIGIN(ram) + LENGTH(ram) - 4);

PHDRS {
	text PT_LOAD FLAGS(7); /* 7 == RWX */
}

SECTIONS
{
	.text :
	{
		*(.text)
		*(.text.*)

		KEEP(*(.init))
		KEEP(*(.fini))

		/* .ctors */
		*crtbegin.o(.ctors)
		*crtbegin?.o(.ctors)
		*(EXCLUDE_FILE(*crtend?.o *crtend.o) .ctors)
		*(SORT(.ctors.*))
		*(.ctors)

		/* .dtors */
		*crtbegin.o(.dtors)
		*crtbegin?.o(.dtors)
		*(EXCLUDE_FILE(*crtend?.o *crtend.o) .dtors)
		*(SORT(.dtors.*))
		*(.dtors)

		*(.rodata*)

		KEEP(*(.eh_frame*))
	} :text /* PUT IT IN THE text SEGMENT */

	.data :
	{
		__etext = .;

		__data_start__ = .;
		*(vtable)
		*(.data*)

		. = ALIGN(4);
		/* preinit data */
		PROVIDE_HIDDEN (__preinit_array_start = .);
		KEEP(*(.preinit_array))
		PROVIDE_HIDDEN (__preinit_array_end = .);

		. = ALIGN(4);
		/* init data */
		PROVIDE_HIDDEN (__init_array_start = .);
		KEEP(*(SORT(.init_array.*)))
		KEEP(*(.init_array))
		PROVIDE_HIDDEN (__init_array_end = .);

		. = ALIGN(4);
		/* fini data */
		PROVIDE_HIDDEN (__fini_array_start = .);
		KEEP(*(SORT(.fini_array.*)))
		KEEP(*(.fini_array))
		PROVIDE_HIDDEN (__fini_array_end = .);

		. = ALIGN(4);
		/* All data end */
		__data_end__ = .;
	} :text /* PUT IT IN THE text SEGMENT */

	.bss :
	{
		. = ALIGN(4);
		__bss_start__ = .;
		*(.bss*)
		*(COMMON)
		*(COM)
		. = ALIGN(4);
		__bss_end__ = .;
	} :text /* PUT IT IN THE text SEGMENT */

	.reloc : {
		. = ALIGN(4);
		/* . = RELOC_TABLE_START; */
		*(.rel*.*) /* might need to include other sections based on compiler output */
	} :text /* PUT IT IN THE text SEGMENT */

	/DISCARD/ :
	{
		*(.ARM.exidx)
	}
}
