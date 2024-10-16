.thumb

.autoregion
.func Timer_SleepFrame
    push {r0-r7, lr}
    ; 启动计时器
    
    ldr r1,=0x4000100
    ldr r0
    
    pop {r0-r7, pc}
.endfunc
.pool
.endautoregion
