### Introduction
Convert format described in https://github.com/riscv/riscv-opcodes/blob/master/opcodes-rvv into Rust code.

### Usage
```bash
echo "vlm.v          31..28=0 27..26=0 25=1 24..20=0xb rs1 14..12=0x0  vd 6..0=0x07" | cargo run
```
