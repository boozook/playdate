/* Elf layout that acceptable for PDC
	docs: https://sourceware.org/binutils/docs/ld/Scripts.html
*/

/* TODO: LD_FEATURE(string) */


ASSERT(CONSTANT(MAXPAGESIZE), "MAXPAGESIZE not set")
ASSERT(CONSTANT(COMMONPAGESIZE), "COMMONPAGESIZE not set")

ASSERT(DEFINED(eventHandlerShim), "Playdate event-handler (entry-point) not found")

ENTRY(eventHandlerShim)

MEMORY
{
	/*
		https://sourceware.org/binutils/docs/ld/MEMORY.html
		RevA load loc: 61800
		Heap: 8388208
	*/
	/* ram (rwx) : ORIGIN = 0, LENGTH = 61800 */
	ram (rwx) : ORIGIN = 0, LENGTH = 8388208
}
PROVIDE(_fstack = ORIGIN(ram) + LENGTH(ram) - 4);

PHDRS {
	/* https://sourceware.org/binutils/docs/ld/PHDRS.html */
	text PT_LOAD FLAGS(7); /* 7=RWX */
}

SECTIONS
{
	.text ALIGN (CONSTANT (MAXPAGESIZE)) :
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


	/* Optional extension like 'plugin-list as section' */
	.pdext ALIGN(4): {
	   __start_pdext = .;
		KEEP(*(.pdext))
	   KEEP(*(.pdext.*))
	   __end_pdext = .;
	   . = ALIGN(4);
   } :text


	/* XXX: better, but expensive:
		.data DATA_SEGMENT_ALIGN(CONSTANT(MAXPAGESIZE), CONSTANT(COMMONPAGESIZE)) : */
	.data DATA_SEGMENT_ALIGN(CONSTANT(COMMONPAGESIZE), 4) :
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

	.bss (NOLOAD) : ALIGN(4)
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
		/* Unused exception related info */
		*(.ARM.exidx);
		*(.ARM.exidx.*);
		*(.ARM.extab.*);
	}
}
