.nds
.thumb
.open TEMP+"/overlay/overlay_0201.bin",readu32(TEMP+"/y9.bin", 201 * 0x20 + 0x4)

; sub_217B6EC
.org 0x217B6EC
    push {r3-r7, lr}
    mov r0, r1
    mov r1, r2
    mov r2, r3
    blx fontapi_sub_217BBC8_hook
    pop {r3-r7, pc}
    
.close