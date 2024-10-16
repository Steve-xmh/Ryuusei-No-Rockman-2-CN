.nds
.thumb
.open TEMP+"/overlay/overlay_0174.bin",readu32(TEMP+"/y9.bin", 174 * 0x20 + 0x4)

; sub_215943C
; 此处会绘制文字，会测量文字绘制宽度
.org 0x02159460
    mov r0, 12
    
.close