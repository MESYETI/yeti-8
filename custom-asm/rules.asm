#once

#subruledef r8
{
	a => 0b000
	b => 0b001
	c => 0b010
	d => 0b011
	e => 0b100
	f => 0b101
	g => 0b110
	h => 0b111
}

#subruledef r16
{
	ab => 0b00
	cd => 0b01
	ef => 0b10
	sp => 0b11
}

#ruledef
{
	halt => 0x00
	nop  => 0x01

	mov {rd: r8}, {rs: r8}          => 0x10 @ rd @ rs @ 0b00
	mov {pd: r16}, {ps: r16}        => 0x11 @ pd @ ps @ 0b0000
	mov {rd: r8}, {value: u8}       => 0x12 @ rd @ 0b00000 @ value
	mov {pd: r16}, {value: u16}     => 0x13 @ pd @ 0b000000 @ $le(value)
	mov {rd: r8}, [{ps: r16}]       => 0x14 @ rd @ ps @ 0b000
	mov {pd: r16}, [{ps: r16}]      => 0x15 @ pd @ ps @ 0b0000
	mov {rd: r8}, [{value: u16}]    => 0x17 @ rd @ 0b00000 @ $le(value)
	mov {pd: r16}, [{value: u16}]   => 0x18 @ pd @ 0b000000 @ $le(value)
	mov [{pd: r16}], {rs: r8}       => 0x19 @ pd @ rs @ 0b000
	mov [{pd: r16}], {ps: r16}      => 0x1A @ pd @ ps @ 0b0000
	mov.b [{pd: r16}], {value: u8}  => 0x1B @ pd @ 0b000000 @ value
	mov.w [{pd: r16}], {value: u16} => 0x1C @ pd @ 0b000000 @ $le(value)

	jmp {value: u16}   => 0x20 @ $le(value)
	jmp [{value: u16}] => 0x21 @ $le(value)
	jmp {pd: r16}      => 0x22 @ pd @ 0b000000
	jmp [{pd: r16}]    => 0x23 @ pd @ 0b000000

	call {value: u16}   => 0x24 @ $le(value)
	call [{value: u16}] => 0x25 @ $le(value)
	call {pd: r16}      => 0x26 @ pd @ 0b000000
	call [{pd: r16}]    => 0x27 @ pd @ 0b000000

	jz {value: u16}  => 0x28 @ $le(value)
	jnz {value: u16} => 0x29 @ $le(value)
	js {value: u16}  => 0x2A @ $le(value)
	jns {value: u16} => 0x2B @ $le(value)
	jc {value: u16}  => 0x2C @ $le(value)
	jnc {value: u16} => 0x2D @ $le(value)

	ret => 0x2E

	cmp {rd: r8}, {rs: r8}    => 0x40 @ rd @ rs @ 0b00
	cmp {rd: r8}, {value: u8} => 0x41 @ rd @ 0b00000 @ value
	cmp {pd: r16}, {ps: r16}  => 0x42 @ pd @ ps @ 0b000000

	add {rd: r8}, {rs: r8} => 0x43 @ rd @ rs @ 0b00
	sub {rd: r8}, {rs: r8} => 0x44 @ rd @ rs @ 0b00
	mul {rd: r8}, {rs: r8} => 0x45 @ rd @ rs @ 0b00
	div {rd: r8}, {rs: r8} => 0x46 @ rd @ rs @ 0b00

	add {pd: r16}, {rs: r8} => 0x47 @ pd @ rs @ 0b000
	sub {pd: r16}, {rs: r8} => 0x48 @ pd @ rs @ 0b000

	and {rd: r8}, {rs: r8} => 0x49 @ rd @ rs @ 0b00
	or  {rd: r8}, {rs: r8} => 0x4A @ rd @ rs @ 0b00
	xor {rd: r8}, {rs: r8} => 0x4B @ rd @ rs @ 0b00

	not {rd: r8} => 0x4C @ rd @ 0b00000

	icmp {rd: r8}, {rs: r8}    => 0x4D @ rd @ rs @ 0b00
	icmp {rd: r8}, {value: u8} => 0x4E @ rd @ 0b00000 @ value
	icmp {pd: r16}, {ps: r16}  => 0x4F @ pd @ ps @ 0b000000

	setz => 0x50
	clz  => 0x51
	sets => 0x52
	cls  => 0x53
	setc => 0x54
	clc  => 0x55

	inc {rd: r8}  => 0x60 @ rd @ 0b00000
	inc {pd: r16} => 0x61 @ pd @ 0b000000
	dec {rd: r8}  => 0x62 @ rd @ 0b00000
	dec {pd: r16} => 0x63 @ pd @ 0b000000

	shl {rd: r8},  {value: u4} => 0x64 @ rd @ 0b0  @ value
	shl {pd: r16}, {value: u4} => 0x65 @ pd @ 0b00 @ value

	shr {rd: r8},  {value: u4} => 0x66 @ rd @ 0b0  @ value
	shr {pd: r16}, {value: u4} => 0x67 @ pd @ 0b00 @ value

	shl {rd: r8}, {rs: r8}  => 0x68 @ rd @ rs @ 0b00
	shl {pd: r16}, {rs: r8} => 0x69 @ pd @ rs @ 0b000

	shr {rd: r8}, {rs: r8}  => 0x6A @ rd @ rs @ 0b00
	shr {pd: r16}, {rs: r8} => 0x6B @ pd @ rs @ 0b000
}

ROM       = 0x0000
CART      = 0x4000
RAM       = 0x8000
VRAM      = 0xC000
VRAM_FONT = 0xCC00

#ruledef
{
	ref {label: u16} => label
}

rom = 0

#if rom == 1
{
	#bankdef rom
	{
		bits = 8
		addr = 0x0000
		size = 0x4000
		outp = 0
	}
}
#else
{
	#bankdef rom
	{
		bits = 8
		addr = 0x4000
		size = 0x4000
		outp = 0
	}
}

#bankdef ram
{
	bits = 8
	addr = 0x8000
	size = 0x4000
}

#bank rom
