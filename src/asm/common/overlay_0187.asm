.nds
.thumb
.open TEMP+"/overlay/overlay_0187.bin",readu32(TEMP+"/y9.bin", 187 * 0x20 + 0x4)

; mem.input.bin sub_217D1E8
.org 0x0217D1E8
.area 0x0217D22A-., 0x00
    push {r4, lr}
    mov r0, r1
    mov r1, r2
    mov r2, r3
    blx fontapi_get_encode_size_by_index
    pop {r4, pc}
.endarea

; mem.input.bin sub_217BA6C
.org 0x0217BA80
.area 0x0217BAAA-., 0x00
    
    push {r0}
    mov r0, r3
    push {r1}
    blx fontapi_get_code_size
    pop {r1}
@@CopyLoop:
    ldrb r3, [r1]
    strb r3, [r4]
    add r1, 1
    add r4, 1
    sub r0, 1
    cmp r0, 0
    bne @@CopyLoop
    
    pop {r0}
    
    add r5, 1
    b 0x0217BAAA
.pool
.endarea

; mem.input.bin sub_217B9E4
.org 0x0217B9FE
.area 0x0217BA0A-., 0x00
    bl sub_217B9E4_hook
    bne 0x0217B9F8
    b 0x0217BA0A
.endarea

; mem.input.bin update_char_sub_217CA68
.org 0x0217CA68
.area 0x0217CACC-., 0x00
    push {r4, lr}
    sub sp, 4
    mov r0, 1
    str r0, [sp]
    mov r0, r1
    mov r1, r2
    mov r2, r3
    ldr r3, [sp, 0xC]
    blx fontapi_input_char
    add sp, 4
    pop {r4, pc}
.endarea

; mem.input.bin sub_217BBEC
.org 0x0217BC08
.area 0x0217BC0C-., 0x00
    bl sub_217BBEC_hook
.endarea

; mem.input.bin sub_217D22C
.org 0x0217D254
.area 0x0217D2CA-., 0x00
    ldrb r1, [r0]
    cmp r1, 0xE4
    bhi 0x0217D2CC
    
    push {r4}

    push {r0}
    mov r0, r1
    blx fontapi_get_code_size
    mov r1, r0
    mov r4, r0
    pop {r0}

@@CheckLoop:
    sub r1, 1
    ldrb r2, [r0, r1]
    ldrb r3, [r7, r1]
    cmp r2, r3
    bne @@End
    cmp r1, 0
    bne @@CheckLoop

    pop {r4}
    mov r0, r4
    mov r1, 0xB
    blx 0x020AD358 ; s32_div_f
    mov r7, r0
    mov r0, r4
    mov r1, 0xB
    blx 0x020AD358 ; s32_div_f
    lsl r0, r1, 2
    add r0, r1
    add r0, r7
    strh r0, [r6, 4]
    str r5, [r6]
    b 0x0217D2DA

@@End:
    add r0, r4
    pop {r4}
    add r4, 1
    b 0x0217D254
.endarea

.close
