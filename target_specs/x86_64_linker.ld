ENTRY(_start)

ENTRY(_start)

SECTIONS
{
	. = 1M;

        .text  : ALIGN(0x1000)
	{
		*(.text.*)
	}

	.rodata : ALIGN(0x1000)
	{
		*(.rodata.*)
	}

	.data : ALIGN(0x1000)
	{
		*(.data.*)
	}

	.bss : ALIGN(0x1000)
	{
		*(.bss.*)
	}
}
