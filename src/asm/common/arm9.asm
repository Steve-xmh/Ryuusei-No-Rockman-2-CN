.org 0x020B7898+0x40*11 ; 8x16 细字体位置
; Global_Patch_Code_Start_Font1:
.region 0x40 * 0x1E1
.endregion
; Global_Patch_Code_End_Font1:

.org 0x020C0098+0x40*2 ; 8x16 粗字体位置
; Global_Patch_Code_Start_Font2:
.region 0x40 * 0x1E1
.endregion
; Global_Patch_Code_End_Font2:

.org 0x020C8898+0x80*2 ; 12x12 字体位置
; Global_Patch_Code_Start_Font3:
.region 0x80 * 0x1E1
.endregion
; Global_Patch_Code_End_Font3:
Global_Zig_Heap_End:

.autoregion
.arm
.align
.importobj "ninja/rust-code.bin"
;; .importobj "ninja/zig-code.bin"
.thumb
.endautoregion

.autoregion
.align
.func sub_218B9B0_hook
    push {r0-r3, r6-r7, lr}
    cmp r0, 0xD0
    bge @ToDoubleEncode
    add r4, 1
    add r5, 1
    pop {r0-r3, r6-r7, pc}
@ToDoubleEncode:
    add r4, 2
    add r5, 2
    pop {r0-r3, r6-r7, pc}
.endfunc
.endautoregion

.autoregion
.align
.func sub_217B9E4_hook
    push {r0, r2, r4-r7, lr}
    
    push {r1, r3}
    mov r0, r5
    blx fontapi_get_code_size
    pop {r1, r3}
    
    cmp r0, 1
    beq @@End
    add r1, r1, 1
    lsl r1, r1, 0x18
    add r3, r3, 2
    lsr r1, r1, 0x18
@@End:
    pop {r0, r2, r4-r7, pc}
.endfunc
.endautoregion

.autoregion
.align
.func sub_217BBEC_hook
    push {r0-r2, r4-r7, lr}
    mov r4, r3
    ldrb r0, [r4]
    blx fontapi_get_code_size
    add r3, r0, r4
    pop {r0-r2, r4-r7, pc}
.endfunc
.endautoregion

.autoregion
.align 4
RustAPI_InitData:
    .dw 0x020B7898 ; font1_pos
    .dw 0x020C0098 ; font2_pos
    .dw 0x020C8898 ; font3_pos
    .dw filesize("../../_temp/fonts/font3_width.bin"); font3_graph_amount
    .dw Global_Zig_Heap_Start ; heap_start
    .dw Global_Zig_Heap_End   ; heap_end
.endautoregion

; ======= 通用的 Hook 代码们 =======
; 以下代码均确认双版本的 ARM9 代码里位置一致

; sub_2002E0C
.org 0x02002E0C
    push {r4, lr}
    blx fontapi_sub_2002E0C_hook
    pop {r4, pc}

; sub_2002F18
.org 0x02002F18
    push {r3-r7, lr}
    blx fontapi_shrink_script_data_sub_2002F18
    pop {r3-r7, pc}

.org 0x0200E8D8
get_archive_path_sub_200E8D8:

.org 0x0200E8A0 ; 调用了 FS_Init
FS_Init_Callee:
    b .

; copy_font3_sub_2026094
.org 0x02026094
copy_font3_sub_2026094:
.org 0x020260FC
    mov r0, 1 ; 强制让复制字形的函数取用第二个字形
.org 0x020260B4
.area 0x020260C2-., 0x00
    push {r0-r7}
    blx fontapi_move_draw_cursor
    pop {r0-r7}
    b 0x020260C2
.endarea

; sub_201ACA4
.org 0x0201AD5A
.area 0x0201ADA0-., 0x00
    ldr r1, [r5, 0x50]
    mov r0, 0x2 ; MOVS R0, #0x2000
    lsl r0, 0xC
    orr r0, r1
    str r0, [r5, 0x50]
    mov r0, r5
    push {r1-r3}
    blx fontapi_read_script_font_3
    pop {r1-r3}
    mov r0, r5
    bl copy_font3_sub_2026094
    b 0x0201ADA0
.endarea

; sub_201AE48
.org 0x0201AE96
    ; 部分 0x0201AE9A 的指令
    ; v7 = 1;
    mov r0, 1
    str r0, [sp]
    ; *v2 |= 0x2000u;
    ldr r1, [r4]
    lsl r0, 0xD
    orr r0, r1
    str r0, [r4]
    ; ++*v3;
    ldrh r0, [r6]
    add r0, 1
    strh r0, [r6]
    
    mov r0, r5
    push {r1-r3}
    blx fontapi_read_script_font_3
    pop {r1-r3}
    mov r0, r5
    bl copy_font3_sub_2026094
    b 0x0201AEE6

; sub_201B438
.org 0x0201B468
    mov r0, r4
    push {r1-r3}
    blx fontapi_read_script_font_3_fixed_width
    pop {r1-r3}
    mov r0, r4
    bl copy_font3_sub_2026094
    b 0x0201B490

; sub_201B578
.org 0x0201B5D8
    mov r0, r4
    push {r1-r3}
    blx fontapi_read_script_font_3
    pop {r1-r3}
    mov r0, r4
    bl copy_font3_sub_2026094
    b 0x0201B590

; sub_201B6E0
.org 0x0201B710
    mov r0, r4
    push {r1-r3}
    blx fontapi_read_script_font_3_fixed_width
    pop {r1-r3}
    mov r0, r4
    bl copy_font3_sub_2026094
    b 0x0201B738

; sub_201D04C
.org 0x0201D086
    mov r0, r5
    push {r1-r3}
    blx fontapi_read_script_font_3
    pop {r1-r3}
    mov r0, r5
    bl copy_font3_sub_2026094
    b 0x0201D0A6

; sub_201B820
.org 0x0201B880
    mov r0, r4
    push {r1-r3}
    blx fontapi_read_script_font_3
    pop {r1-r3}
    mov r0, r4
    bl copy_font3_sub_2026094
    b 0x0201B838

; sub_201B984
.autoregion
.align
.func sub_201B984_hook
    push {r1-r4, r6-r7, lr}
    mov r0, r1
    push {r5}
    push {r1-r3}
    blx fontapi_get_code_size
    pop {r1-r3}
    pop {r5}
    add r5, r0
    pop {r1-r4, r6-r7, pc}
.endfunc
.endautoregion
.org 0x0201BA34
.area 0x0201BA40-.
    bl sub_201B984_hook
    b 0x0201B98C
.endarea

; sub_2027140
.org 0x0202717A
    mov r0, r5
    push {r1-r3}
    blx fontapi_read_script_font
    pop {r1-r3}
    b 0x02027194
.org 0x020271B8
    mov r2, 1

; sub_2027444
.org 0x0202747E
    mov r0, r5
    blx fontapi_read_script
    b 0x02027498

; sub_2038CBC
.org 0x02038CF4
    mov r0, r5
    push {r1-r3}
    blx fontapi_read_script_font_3
    pop {r1-r3}
    mov r0, r5
    bl copy_font3_sub_2026094
    b 0x02038CC8

; sub_2002DB4
.org 0x02002DC8
    push {lr}
    mov r0, r2
    blx fontapi_get_script_chars_len_loop
    pop {pc}

; sub_2002DE8
.org 0x02002DE8
    push {lr}
    blx fontapi_get_script_chars_len
    pop {pc}

; sub_2002C3C
.org 0x02002C3C
    push {lr}
    blx fontapi_get_font_graph_addr
    pop {pc}

// 这个代码可以帮助输出读取归档文件的信息
; get_archive_path_sub_200E8D8
.org 0x0200E8DA
    bl get_archive_path_sub_200E8D8_patch
.autoregion
.align
.func get_archive_path_sub_200E8D8_patch
    push {lr}
    push {r0-r7}
    mov r0, r1
    ; get callee
    ldr r1, [sp, 12 * 4]
    blx fontapi_log_get_archive_path
    pop {r0-r7}
    ldr r2, =0xFFFF0000
    add r5, r0, 0x0
    pop {pc}
.pool
.endfunc
.endautoregion

.autoregion 0x020C8898
.align
Global_Zig_Heap_Start:
.if Global_Zig_Heap_Start > Global_Zig_Heap_End
.error "Global_Zig_Heap_Start > Global_Zig_Heap_End"
.endif
.endautoregion