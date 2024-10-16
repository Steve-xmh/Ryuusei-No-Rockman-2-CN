.nds
.thumb
.open TEMP+"/overlay/overlay_0184.bin",readu32(TEMP+"/y9.bin", 184 * 0x20 + 0x4)

; sub_217AB78
; 原本会裁切值到 uint16_t，但是这里不需要
.org 0x0217AC1A
    mov r1, r0
    nop

; sub_217AC6C
.org 0x0217AC6C
.area 0x0217AC78-., 0x00
    push {lr}
    sub r1, 2
    blx fontapi_place_char
    pop {pc}
.endarea

.org 0x0217AAEA
    blx fontapi_reset_vram_cache_folder
    
.close