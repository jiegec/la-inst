#include <stdio.h>
#include <stdint.h>

uint32_t cpucfg(int index) {
  uint32_t res;
  asm volatile("cpucfg %0, %1" : "=r"(res) : "r"(index));
  return res;
}

int main() {
  uint32_t cpucfg0 = cpucfg(0);
  printf("CPUCFG0=0x%08lx\n", cpucfg0);
  printf("  PRID=0x%08lx\n", cpucfg0);

  uint32_t cpucfg1 = cpucfg(1);
  printf("CPUCFG1=0x%08lx\n", cpucfg1);
  char *arch[] = {"LA32R", "LA32", "LA64", "RSV"};
  printf("  ARCH=0b%02lb(%s)\n", cpucfg1 & 0b11, arch[cpucfg1 & 0b11]);
  printf("  PGMMU=0b%01lb\n", (cpucfg1 >> 2) & 0b1);
  printf("  IOCSR=0b%01lb\n", (cpucfg1 >> 3) & 0b1);
  uint32_t palen = (cpucfg1 >> 4) & 0b11111111;
  printf("  PALEN=0x%03lx(%d)\n", palen, palen + 1);
  uint32_t valen = (cpucfg1 >> 12) & 0b11111111;
  printf("  VALEN=0x%03lx(%d)\n", valen, valen + 1);
  printf("  UAL=0b%01lb\n", (cpucfg1 >> 20) & 0b1);
  printf("  RI=0b%01lb\n", (cpucfg1 >> 21) & 0b1);
  printf("  EP=0b%01lb\n", (cpucfg1 >> 22) & 0b1);
  printf("  RPLV=0b%01lb\n", (cpucfg1 >> 23) & 0b1);
  printf("  HP=0b%01lb\n", (cpucfg1 >> 24) & 0b1);
  printf("  CRC=0b%01lb\n", (cpucfg1 >> 25) & 0b1);
  printf("  MSG_INT=0b%01lb\n", (cpucfg1 >> 26) & 0b1);
  printf("  RSV27=0b%05lb\n", cpucfg1 >> 27);

  uint32_t cpucfg2 = cpucfg(2);
  printf("CPUCFG2=0x%08lx\n", cpucfg2);
  printf("  FP=0b%01lb\n", cpucfg2 & 0b1);
  printf("  FP_SP=0b%01lb\n", (cpucfg2 >> 1) & 0b1);
  printf("  FP_DP=0b%01lb\n", (cpucfg2 >> 2) & 0b1);
  printf("  FP_ver=0b%01lx\n", (cpucfg2 >> 3) & 0b111);
  printf("  LSX=0b%01lb\n", (cpucfg2 >> 6) & 0b1);
  printf("  LASX=0b%01lb\n", (cpucfg2 >> 7) & 0b1);
  printf("  COMPLEX=0b%01lb\n", (cpucfg2 >> 8) & 0b1);
  printf("  CRYPTO=0b%01lb\n", (cpucfg2 >> 9) & 0b1);
  printf("  LVZ=0b%01lb\n", (cpucfg2 >> 10) & 0b1);
  printf("  LVZ_ver=0b%01lx\n", (cpucfg2 >> 11) & 0b111);
  printf("  LLFTP=0b%01lb\n", (cpucfg2 >> 14) & 0b1);
  printf("  LLFTP_ver=0b%01lx\n", (cpucfg2 >> 15) & 0b111);
  printf("  LBT_X86=0b%01lb\n", (cpucfg2 >> 18) & 0b1);
  printf("  LBT_ARM=0b%01lb\n", (cpucfg2 >> 19) & 0b1);
  printf("  LBT_MIPS=0b%01lb\n", (cpucfg2 >> 20) & 0b1);
  printf("  LSPW=0b%01lb\n", (cpucfg2 >> 21) & 0b1);
  printf("  LAM=0b%01lb\n", (cpucfg2 >> 22) & 0b1);
  // CPUCFG2[23] undefined
  printf("  RSV23=0b%01lb\n", (cpucfg2 >> 23) & 0b1);
  printf("  HPTW=0b%01lb\n", (cpucfg2 >> 24) & 0b1);
  printf("  FRECIPE=0b%01lb\n", (cpucfg2 >> 25) & 0b1);
  printf("  DIV32=0b%01lb\n", (cpucfg2 >> 26) & 0b1);
  printf("  LAM_BH=0b%01lb\n", (cpucfg2 >> 27) & 0b1);
  printf("  LAMCAS=0b%01lb\n", (cpucfg2 >> 28) & 0b1);
  printf("  LLACQ_SCREL=0b%01lb\n", (cpucfg2 >> 29) & 0b1);
  printf("  SCQ=0b%01lb\n", (cpucfg2 >> 30) & 0b1);
  printf("  RSV31=0b%01lb\n", (cpucfg2 >> 31) & 0b1);

  uint32_t cpucfg3 = cpucfg(3);
  printf("CPUCFG3=0x%08lx\n", cpucfg3);
  printf("  CCDMA=0b%01lb\n", cpucfg3 & 0b1);
  printf("  SFB=0b%01lb\n", (cpucfg3 >> 1) & 0b1);
  printf("  UCACC=0b%01lb\n", (cpucfg3 >> 2) & 0b1);
  printf("  LLEXC=0b%01lb\n", (cpucfg3 >> 3) & 0b1);
  printf("  SCDLY=0b%01lb\n", (cpucfg3 >> 4) & 0b1);
  printf("  LLDBAR=0b%01lb\n", (cpucfg3 >> 5) & 0b1);
  printf("  ITLBHMC=0b%01lb\n", (cpucfg3 >> 6) & 0b1);
  printf("  ICHMC=0b%01lb\n", (cpucfg3 >> 7) & 0b1);
  printf("  SPW_LVL=0b%01lx\n", (cpucfg3 >> 9) & 0b111);
  printf("  SPW_HP_HF=0b%01lb\n", (cpucfg3 >> 10) & 0b1);
  printf("  RVA=0b%01lb\n", (cpucfg3 >> 12) & 0b1);
  printf("  RVAMAX_MINUS_1=0b%01lb\n", (cpucfg3 >> 13) & 0b11111);
  printf("  DBAR_hints=0b%01lb\n", (cpucfg3 >> 17) & 0b1);
  // CPUCFG3[22:18] undefined
  printf("  RSV18=0b%03lb\n", (cpucfg3 >> 18) & 0b11111);
  printf("  LD_SEQ_SA=0b%01lb\n", (cpucfg3 >> 23) & 0b1);
  printf("  RSV24=0b%03lb\n", cpucfg3 >> 24);
  return 0;
}
