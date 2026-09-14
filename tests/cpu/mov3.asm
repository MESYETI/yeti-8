mov ef, data8
mov a, [ef]

mov ef, data16
mov cd, [ef]
halt

data8:  #d8 8
data16: #d16 $le(0x0200)
