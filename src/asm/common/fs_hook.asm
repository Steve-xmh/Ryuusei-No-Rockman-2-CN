.thumb
.org FS_Init_Callee
    blx FS_Init_Hook

.autoregion
.arm
.align
.func FS_Init_Hook
    push {r0-r7, lr}
    bl FS_Init
.ifdef zig_init
    ldr r0, =RustAPI_InitData
    bl zig_init
.endif
.ifdef fontapi_main
    ldr r0, =RustAPI_InitData
    bl fontapi_main
.endif
    pop {r0-r7, pc}
.endfunc
.pool
.endautoregion
