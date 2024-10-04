# SPDX-License-Identifier: MIT
        .section .text._pvh_start, "awx"
        .global _pvh_start
        .code32

_pvh_start:
        call com0_init

        mov eax, cr0
        test al, 1
        jnz protected_mode
        
        mov al, 'p'
        call com0_printc
        jmp die

protected_mode: 
        mov al, 'P'
        call com0_printc

check_long_mode:
        mov eax, 0x80000000
        cpuid
        cmp eax, 0x80000001
        jb no_long_mode
        jmp long_64_mode

no_long_mode:
        mov al, 'l'
        call com0_printc
        jmp die

long_64_mode:
        mov al, 'L'
        call com0_printc

        cli

        # Clear tables:
        mov edi, 0x1000
        mov cr3, edi
        xor eax, eax
        mov ecx, 4096
        rep stosd
        mov edi, cr3

        # PML4T - 0x1000 + rw
        # PDPT - 0x2000  + rw
        # PDT - 0x3000   + rw
        # PT - 0x4000    + rw
        mov dword ptr [edi], 0x2003  
        add edi, 0x1000              
        mov dword ptr [edi], 0x3003
        add edi, 0x1000
        mov dword ptr [edi], 0x4003
        add edi, 0x1000

        # 
        mov ebx, 0x00000003
        mov ecx, 512

.set_entry_loop:
        mov dword ptr [edi], ebx
        add ebx, 0x1000
        add edi, 8
        loop .set_entry_loop

        # PAE-paging
        mov eax, cr4
        or eax, 1 << 5    # PAE-bit
        mov cr4, eax

        # Set long mode bit
        mov ecx, 0xC0000080 # EFER MSR
        rdmsr
        or eax, (1 << 8)    # LM-bit
        wrmsr

        mov al, '.'
        call com0_printc

        # Enabling paging
        mov eax, cr0
        or eax, (1 << 31)
        mov cr0, eax

        mov al, '.'
        call com0_printc

        # Load the 64-bit global descriptor table.
        lgdt gdt_64_pointer

        mov al, '.'
        call com0_printc

        # Jump to Rust entry point
        push 0x8
        mov eax, offset _start
        push eax
        retf

die:
        mov al, 'x'
        call com0_printc
        hlt
        jmp die

        .equ COM1_BASE, 0x3f8
        .equ COM1_LCR, COM1_BASE + 3
        .equ COM1_IER, COM1_BASE + 1
        .equ COM1_FCR, COM1_BASE + 2
        .equ COM1_MCR, COM1_BASE + 4

com0_init:
        mov al, 3
        mov dx, COM1_LCR
        out dx, al

        mov al, 0
        mov dx, COM1_IER
        out dx, al

        mov dx, COM1_FCR
        out dx, al
        
        mov al, 3
        mov dx, COM1_MCR
        out dx, al
        ret
        
com0_printc:
        push dx
        mov dx, COM1_BASE
        out dx, al
        pop dx
        ret

gdt_64:
    .quad 0x0000000000000000          # Null Descriptor - should be present.
    .quad 0x00209A0000000000          # 64-bit code descriptor (exec/read).
    .quad 0x0000920000000000          # 64-bit data descriptor (read/write).

.align 4
    .word 0                              # Padding to make the "address of the GDT" field aligned on a 4-byte boundary

gdt_64_pointer:
    .word gdt_64_pointer - gdt_64 - 1    # 16-bit Size (Limit) of GDT.
    .long gdt_64                         # 32-bit Base Address of GDT. (CPU will zero extend to 64-bit)
