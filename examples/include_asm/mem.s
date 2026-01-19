mem::alloc(u16)->any*:
    read r0 mem::HEAP
    sub r0 [#-3]
    write mem::HEAP r0
    ret

mem::free(any*):
    ret

mem::copy(any*,any*,u16).loop:
    copyitr [#-5] [#-4]
mem::copy(any*,any*,u16):
    jrnzdec [#-3] mem::copy(any*,any*,u16).loop
    ret

mem::HEAP:
     dw #0