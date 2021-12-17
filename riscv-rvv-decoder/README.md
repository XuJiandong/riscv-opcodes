### Introduction
Convert format described in https://github.com/riscv/riscv-opcodes/blob/master/opcodes-rvv into Rust code.

### Usage
```bash
echo "vlm.v          31..28=0 27..26=0 25=1 24..20=0xb rs1 14..12=0x0  vd 6..0=0x07" | cargo run
```
Then get Rust code snippet:
```
x if x & 0b_1111_11_1_11111_00000_111_00000_1111111 == 0b_0000_00_1_01011_00000_000_00000_0000111 => Some(insts::OP_VLM_V)
```
