/* Playdate OS 2.0+ elf layout compatible with PDC (v2.2+).
	Linker: rust-lld, lld, ld.
	Hardware rev: for RevA, compliant with RevB.
*/

ASSERT(DEFINED(eventHandlerShim), "Playdate event-handler (entry-point) not found")
ENTRY(eventHandlerShim)

/*
	https://sourceware.org/binutils/docs/ld/MEMORY.html
	RevA load loc: 61800
	Heap: 8388208 (8MiB, probably "usable safe" could be 12-14M)
	RAM: 16MiB

MEMORY { ram (rwx) : ORIGIN = 0, LENGTH = 8388208 }
PROVIDE(_fstack = ORIGIN(ram) + LENGTH(ram) - 4);
	RWX for JIT:
	https://sourceware.org/binutils/docs/ld/PHDRS.html
PHDRS { text PT_LOAD FLAGS(7); }
*/


SECTIONS
{
	.text ALIGN(4): {
		*(.text .text.*)

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

		*(.rodata .rodata.*)
		*(.srodata .srodata.*)

		*(.eh_frame*)

		. = ALIGN(4);
		__etext = .;
	}


	.data ALIGN(4): {
		__data_start__ = .;
		*(vtable)

		*(.data .data.*)
		*(.sdata .sdata.*)

		. = ALIGN(4);

		/* preinit */
		PROVIDE_HIDDEN (__preinit_array_start = .);
		KEEP(*(.preinit_array))
		PROVIDE_HIDDEN (__preinit_array_end = .);

		. = ALIGN(4);
		/* init */
		PROVIDE_HIDDEN (__init_array_start = .);
		KEEP(*(SORT(.init_array.*)))
		KEEP(*(.init_array))
		PROVIDE_HIDDEN (__init_array_end = .);

		. = ALIGN(4);
		/* fini */
		PROVIDE_HIDDEN (__fini_array_start = .);
		KEEP(*(SORT(.fini_array.*)))
		KEEP(*(.fini_array))
		PROVIDE_HIDDEN (__fini_array_end = .);

		. = ALIGN(4);
		__data_end__ = .;
	}

	.bss (NOLOAD) : ALIGN(4) {
		__bss_start__ = .;
		*(.bss .bss.*)
		*(.sbss .sbss.*)
		*(COMMON)
		*(COM)
		. = ALIGN(4);
		__bss_end__ = .;
	}

	/* optional extension-like list: */
	.pdext ALIGN(4): {
		__start_pdext = .;
		KEEP(*(.pdext))
		KEEP(*(.pdext.*))
		__end_pdext = .;
		. = ALIGN(4);
	}


/* TODO: this should be injectable from user's script, optionally
	`.debug_addr`+reloc needed for DWARFv5 and probably for v4.

	.debug (NOLOAD) : {
		*(.debug*)
	}
*/


	/DISCARD/ :
	{
		/* Unused exception related info */
		*(.ARM.exidx);
		*(.ARM.exidx.*);
		*(.ARM.extab.*);
	}
}
