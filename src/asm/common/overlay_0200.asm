.nds
.thumb
.open TEMP+"/overlay/overlay_0200.bin",readu32(TEMP+"/y9.bin", 200 * 0x20 + 0x4)

; place_char_sub_217C0FC
.org 0x0217C0FC
    push {lr}
    blx fontapi_place_char
    pop {pc}

; sub_217AE30
.org 0x0217AE56
    blx fontapi_reset_vram_cache_library

.close