.nds
.thumb
.open TEMP+"/overlay/overlay_0874.bin",readu32(TEMP+"/y9.bin", 874 * 0x20 + 0x4)

; sub_218B9B0
.org 0x0218BA3C
    bl sub_218B9B0_hook

.close