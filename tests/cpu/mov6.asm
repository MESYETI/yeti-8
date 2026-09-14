mov ab, data16
mov cd, 512
mov [ab], cd

mov ab, [ab]

halt

data16: #d16 0
