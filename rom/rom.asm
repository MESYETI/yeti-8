jmp start

#d 0x00 ; padding
#d16 mem_copy
#d16 key_load_state

start:
	; set up stack
	mov sp, 0xC000

	; set up keyboard
	mov   ab,   0xD002 ; set port A to input
	mov.b [ab], 0

	mov   ab,   0xD003 ; set port B to output
	mov.b [ab], 0xFF

	mov ab, 0xC100
	mov c,  64
	mov ef, header

.printHeader:
	mov g, [ef]
	mov [ab], g
	inc ef
	inc ab
	dec c
	jnz .printHeader

.printRam:
	; TODO: detect RAM expansion
	mov ab, 0xC160
	mov cd, ram_4k
	mov e,  6
	call mem_copy

	mov   ab, 0xC180
	mov.b [ab], ">"

	; boot straight to cartridge
	jmp 0x4000

halt
                      
header: #d " * * * * * YETI-8 ROM * * * * * "
        #d "          MONITOR V0.1          "

ram_4k:  #d "4K RAM"
ram_16k: #d "16K EXPANDED RAM"

; API
mem_copy:
	; AB = addr
	; CD = string
	; E  = len
	mov g, [cd]
	mov [ab], g
	inc ab
	inc cd
	dec e
	jnz mem_copy
	ret

; Keyboard
key_state1: #d8 0, 0, 0, 0, 0, 0, 0
key_state2: #d8 0, 0, 0, 0, 0, 0, 0
key_change: #d8 0, 0, 0, 0, 0, 0, 0

key_load_state:
	; AB = dest
	mov cd, 0xD001 ; column select address
	mov e,  1 ; column
	mov g,  7 ; counter

.loop:
	mov [cd], e        ; select column
	mov f,    [0xD000] ; read key bitmask
	mov [ab], f        ; write bitmask to dest

	shl e, 1
	inc ab
	dec g
	jnz .loop
	ret
