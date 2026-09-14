# CPU instruction set architecture
The Y8010 CPU is an 8-bit CPU with a 16-bit address bus.


## 8-bit Registers
| Number | Name       | Use             |
| ------ | ---------- | --------------- |
| 0      | A          | General purpose |
| 1      | B          | General purpose |
| 2      | C          | General purpose |
| 3      | D          | General purpose |
| 4      | E          | General purpose |
| 5      | F          | General purpose |
| 6      | G          | General purpose |
| 7      | H          | General purpose |

## 16-bit Registers
| Number | Name       | Use             |
| ------ | ---------- | --------------- |
| 0      | AB         | General purpose |
| 1      | CD         | General purpose |
| 2      | EF         | General purpose |
| 3      | SP         | Stack pointer   |

A Program Counter (PC) is also used.

## Flag register
Flags:
- (Z) Zero
- (S) Sign
- (C) Carry

## Instructions

#### Notation
- `Rd`  - 8-bit register destination
- `Rs`  - 8-bit register source
- `N4`  - 4-bit integer literal
- `N8`  - 8-bit integer literal
- `N16` - 16-bit integer literal
- `Pd`  - 16-bit register destination
- `Ps`  - 16-bit register source

`Rd` and `Rs` each store 3 bits.

`N4` stores 4 bits.

`N8` stores 8 bits.

`N16` stores 16 bits.

`Pd` and `Ps` each store 2 bits.

| Opcode (hex) | Opcode (bin) | Parameter format         | Name | Description                    | Flags | Size |
| ------------ | ------------ | ------------------------ | ---- | ------------------------------ | ----- | ---- |
| `00`         | `00000000`   |                          | HALT | Stops the computer             |       | 1    |
| `01`         | `00000001`   |                          | NOP  | Does nothing                   |       | 1    |
| `10`         | `00010000`   | `Rd Rs 00`               | MOV  | `Rd <- Rs`                     | ZS    | 2    |
| `11`         | `00010001`   | `Pd Ps 0000`             | MOV  | `Pd <- Ps`                     | ZS    | 2    |
| `12`         | `00010010`   | `Rd 00000 N8`            | MOV  | `Rd <- N8`                     | ZS    | 3    |
| `13`         | `00010011`   | `Pd 000000 N16`          | MOV  | `Pd <- N16`                    | ZS    | 4    |
| `14`         | `00010100`   | `Rd Ps 000`              | MOV  | `Rd <- [Ps]`                   | ZS    | 2    |
| `15`         | `00010101`   | `Pd Ps 0000`             | MOV  | `Pd <- [Ps]`                   | ZS    | 2    |
| `17`         | `00010111`   | `Rd 00000 N16`           | MOV  | `Rd <- [N16]`                  | ZS    | 3    |
| `18`         | `00011000`   | `Pd 000000 N16`          | MOV  | `Pd <- [N16]`                  | ZS    | 4    |
| `19`         | `00011001`   | `Pd Rs 000`              | MOV  | `[Pd] <- Rs`                   | ZS    | 2    |
| `1A`         | `00011010`   | `Pd Ps 0000`             | MOV  | `[Pd] <- Ps`                   | ZS    | 2    |
| `1B`         | `00011011`   | `Pd 000000 N8`           | MOV  | `[Pd] <- N8`                   | ZS    | 3    |
| `1C`         | `00011100`   | `Pd 000000 N16`          | MOV  | `[Pd] <- N16`                  | ZS    | 4    |
| `20`         | `00100000`   | `N16`                    | JMP  | `PC <- N16`                    |       | 3    |
| `21`         | `00100001`   | `N16`                    | JMP  | `PC <- [N16]`                  |       | 3    |
| `22`         | `00100010`   | `Pd 000000`              | JMP  | `PC <- Pd`                     |       | 2    |
| `23`         | `00100011`   | `Pd 000000`              | JMP  | `PC <- [Pd]`                   |       | 2    |
| `24`         | `00100100`   | `N16`                    | CALL | `push(PC), PC <- N16`          |       | 3    |
| `25`         | `00100101`   | `N16`                    | CALL | `push(PC), PC <- [N16]`        |       | 3    |
| `26`         | `00100110`   | `Pd 000000`              | CALL | `push(PC), PC <- Pd`           |       | 2    |
| `27`         | `00100111`   | `Pd 000000`              | CALL | `push(PC), PC <- [Pd]`         |       | 2    |
| `28`         | `00101000`   | `N16`                    | JZ   | `PC <- N16`                    |       | 3    |
| `29`         | `00101001`   | `N16`                    | JNZ  | `PC <- N16`                    |       | 3    |
| `2A`         | `00101010`   | `N16`                    | JS   | `PC <- N16`                    |       | 3    |
| `2B`         | `00101011`   | `N16`                    | JNS  | `PC <- N16`                    |       | 3    |
| `2C`         | `00101100`   | `N16`                    | JC   | `PC <- N16`                    |       | 3    |
| `2D`         | `00101101`   | `N16`                    | JNC  | `PC <- N16`                    |       | 3    |
| `2E`         | `00101110`   |                          | RET  | `PC <- pop()`                  |       | 1    |
| `40`         | `01000000`   | `Rd Rs 00`               | CMP  | `cmp(Rd, Rs)`                  | ZSC   | 2    |
| `41`         | `01000001`   | `Rd 00000 N8`            | CMP  | `cmp(Rd, N8)`                  | ZSC   | 3    |
| `42`         | `01000010`   | `Pd Ps 0000`             | CMP  | `cmp(Pd, Ps)`                  | ZSC   | 2    |
| `43`         | `01000011`   | `Rd Rs 00`               | ADD  | `Rd <- Rd + Rs`                | ZSC   | 2    |
| `44`         | `01000100`   | `Rd Rs 00`               | SUB  | `Rd <- Rd - Rs`                | ZS    | 2    |
| `45`         | `01000101`   | `Rd Rs 00`               | MUL  | `Rd <- Rd * Rs`                | ZS    | 2    |
| `46`         | `01000110`   | `Rd Rs 00`               | DIV  | `Rd <- Rd / Rs`                | ZS    | 2    |
| `47`         | `01000111`   | `Pd Rs 000`              | ADD  | `Pd <- Pd + Rs`                | ZSC   | 2    |
| `48`         | `01001000`   | `Pd Rs 000`              | SUB  | `Pd <- Pd - Rs`                | ZS    | 2    |
| `49`         | `01001001`   | `Rd Rs 00`               | AND  | `Rd <- Rd & Rs`                | ZS    | 2    |
| `4A`         | `01001010`   | `Rd Rs 00`               | OR   | `Rd <- Rd | Rs`                | ZS    | 2    |
| `4B`         | `01001011`   | `Rd Rs 00`               | XOR  | `Rd <- Rd ^ Rs`                | ZS    | 2    |
| `4C`         | `01001100`   | `Rd 00000`               | NOT  | `Rd <- ~Rd`                    | ZS    | 2    |
| `4D`         | `01001101`   | `Rd Rs 00`               | ICMP | `icmp(Rd, Rs)`                 | ZSC   | 2    |
| `4E`         | `01001110`   | `Rd 00000 N8`            | ICMP | `icmp(Rd, N8)`                 | ZSC   | 3    |
| `4F`         | `01001111`   | `Pd Ps 0000`             | ICMP | `icmp(Pd, Ps)`                 | ZSC   | 2    |
| `50`         | `01010000`   |                          | SETZ | `Z = 1`                        | Z     | 1    |
| `51`         | `01010001`   |                          | CLZ  | `Z = 0`                        | Z     | 1    |
| `52`         | `01010010`   |                          | SETS | `S = 1`                        | S     | 1    |
| `53`         | `01010011`   |                          | CLS  | `S = 0`                        | S     | 1    |
| `54`         | `01010100`   |                          | SETC | `C = 1`                        | C     | 1    |
| `55`         | `01010101`   |                          | CLC  | `C = 0`                        | C     | 1    |
| `60`         | `01100000`   | `Rd 00000`               | INC  | `Rd = Rd + 1`                  | ZC    | 2    |
| `61`         | `01100001`   | `Pd 000000`              | INC  | `Pd = Pd + 1`                  | ZC    | 2    |
| `62`         | `01100010`   | `Rd 00000`               | DEC  | `Rd = Rd - 1`                  | ZC    | 2    |
| `63`         | `01100011`   | `Pd 000000`              | DEC  | `Pd = Pd - 1`                  | ZC    | 2    |
| `64`         | `01100100`   | `Rd 0 N4`                | SHL  | `Rd = Rd << N4`                | Z     | 2    |
| `65`         | `01100101`   | `Pd 00 N4`               | SHL  | `Pd = Pd << N4`                | Z     | 2    |
| `66`         | `01100110`   | `Rd 0 N4`                | SHR  | `Rd = Rd >> N4`                | Z     | 2    |
| `67`         | `01100111`   | `Pd 00 N4`               | SHR  | `Pd = Pd >> N4`                | Z     | 2    |
| `68`         | `01101000`   | `Rd Rs 00`               | SHL  | `Rd = Rd << Rs`                | Z     | 2    |
| `69`         | `01101001`   | `Pd Rs 000`              | SHL  | `Pd = Pd << Rs`                | Z     | 2    |
| `6A`         | `01101010`   | `Rd Rs 00`               | SHR  | `Rd = Rd >> Rs`                | Z     | 2    |
| `6B`         | `01101011`   | `Pd Rs 000`              | SHR  | `Pd = Pd >> Rs`                | Z     | 2    |

### Compare instruction
Sets `Z` for equality, `S` for lesser than, and `C` for greater than.
