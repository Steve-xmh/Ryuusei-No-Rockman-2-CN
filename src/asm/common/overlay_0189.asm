.nds
.thumb
.open TEMP+"/overlay/overlay_0189.bin",readu32(TEMP+"/y9.bin", 189 * 0x20 + 0x4)

; sub_217BBC8
.org 0x217BBC8
    push {r3-r7, lr}
    mov r0, r1
    mov r1, r2
    mov r2, r3
    blx fontapi_sub_217BBC8_hook
    pop {r3-r7, pc}
    
.close