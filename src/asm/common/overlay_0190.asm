.nds
.thumb
.open TEMP+"/overlay/overlay_0190.bin",readu32(TEMP+"/y9.bin", 190 * 0x20 + 0x4)

; sub_217E07C
.org 0x0217E07C
    push {r4 - r6, lr}
    mov r0, r1
    mov r1, r2
    mov r2, r3
    blx fontapi_transform_multi_line_script
    pop {r4 - r6, pc}

.close