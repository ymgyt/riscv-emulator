# riscv-emulator

`riscv-emulator` is a [RISC-V](https://riscv.org/)  emulator.  
This is a hands-on project for [RustでRISC-Vエミュレータを書いてNOMMU Linuxをブラウザで動かした](https://bokuweb.github.io/undefined/articles/20230523.html) blog post.

The implementation targets are

* I(Base Instruction Set)
* M
* A
* Zicsr


## TODO

### RV32I Base Instruction Set

- [x] LUI
- [x] AUIPC
- [x] JAL
- [x] JALR
- [x] BEQ
- [x] BNE
- [x] BLT
- [x] BGE
- [x] BLTU
- [x] BGEU
- [x] LB
- [x] LH
- [x] LW
- [x] LBU
- [ ] SB
- [ ] SH
- [ ] SW
- [ ] ADDI
- [ ] SLTI
- [ ] SLTIU
- [ ] XORI
- [ ] ORI
- [ ] ANDI
- [ ] SLLI
- [ ] SRLI
- [ ] SRAI
- [ ] ADD
- [ ] SUB
- [ ] SLL
- [ ] SLT
- [ ] SLTU
- [ ] XOR
- [ ] SRL
- [ ] SRA
- [ ] OR
- [ ] AND
- [ ] FENCE
- [ ] ECALL
- [ ] EBREAK


### Zicsr

- [x] Csrrw,
- [ ] Csrrs,
- [ ] Csrrc,
- [ ] Csrrwi,
- [ ] Csrrsi,
- [ ] Csrrci,
