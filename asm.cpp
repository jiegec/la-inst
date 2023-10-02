#include <stdint.h>
#include <stdio.h>
#include <string.h>

int arr[16];
int main() {
  for (int i = 0; i < 16; i++)
    arr[i] = 0x87868584 + i;
    // rd = 16, rj = 15, rk = 17
    // ld.b 00111 0000 1011 0010 xxxxx rj rd
#define INSTR 0b00111000010111010100010111110000
#define SNST "0b00111000010111010100010111110000"
  unsigned int inst = INSTR;
  printf("Inst: 0x%x 0b%032b\n", inst, inst);
  printf("rd: %d\n", inst & 0x1f);
  printf("rj: %d\n", (inst >> 5) & 0x1f);
  printf("rk: %d\n", (inst >> 10) & 0x1f);
  // rd = mem[rj]:1
  register uint64_t addr asm("r15") = (uint64_t)(&arr[0]) + 2;
  register uint64_t result asm("r16") = 0;
  register uint64_t offset asm("r17") = 0x1122;
  asm volatile(".word " SNST : "=r"(result) : "r"(addr) : "memory");
  // asm volatile ("ld.bu %0, %1, 0" : "=r"(result) : "r"(addr) : "memory");
  printf("%lx\n", result);
  for (int i = 0; i < 16; i++)
    if (arr[i] != 0x87868584 + i)
      printf("%d changed to %x\n", i, arr[i]);
}
