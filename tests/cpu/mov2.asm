mov a, [data8]
mov cd, [data16]
halt

data8:  #d8 8
data16: #d16 $le(0x0200)
