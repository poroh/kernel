# SPDX-License-Identifier: MIT
        .section .pvh.note, "a", @note
        .align 4
        .long  4      # name size
        .long  8      # desc size
        .long  18     # type
        .asciz "Ker"  # name
        .align 4
        .quad  _pvh_start
        .align 4


