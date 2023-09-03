#include <sys/mman.h>
#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sched.h>
#include <unistd.h>
#include <sys/ptrace.h>
#include <sys/user.h>
#include <sys/wait.h>
#include <sys/uio.h>
#include <elf.h>
#include <sys/mman.h>

// https://github.com/RensDofferhoff/iScanU

int traceeSetup(void* ptr) {
        for (;;) {
                ptrace(PTRACE_TRACEME, 0, 0, 0);
        }
        return -1; //should never reach
}

int main() {
        siginfo_t sig;
        int pageSize = 16384;
        void* traceeStack = (uint8_t*)mmap(NULL, pageSize, PROT_WRITE | PROT_READ, MAP_SHARED | MAP_ANONYMOUS | MAP_NORESERVE, 0, 0) + pageSize;

        void *instructionPointer = (uint8_t*)mmap(NULL, pageSize, PROT_WRITE | PROT_READ | PROT_EXEC, MAP_PRIVATE | MAP_ANONYMOUS | MAP_NORESERVE, 0, 0);
        //addi.d          $t1, $t1, -44
        // unsigned int inst = 0x02ff51ad;
        for (;;) {
                // unsigned int inst = 0x10 | 0x20 | (0b10000000 << 10);
                // unsigned int inst = rand();
                unsigned int inst = 0b00111000010110000000000111110000;
                memcpy(instructionPointer, &inst, 4);
                printf("Inst: 0x%x 0b%032b\n", inst, inst);
                printf("rd: %d\n", inst & 0x1f);
                printf("rj: %d\n", (inst >> 5) & 0x1f);
                printf("rk: %d\n", (inst >> 10) & 0x1f);
                FILE *fp = fopen("temp", "wb");
                fwrite(&inst, 1, sizeof(inst), fp);
                fflush(fp);
                fclose(fp);

                int ret = system("objdump -b binary -m Loongarch64 -D temp | grep word >/dev/null");
                int binutils_illegal = (ret == 0);
                // int ret = 0;
                // int binutils_illegal = 0;
                if (ret == 0) {
                        printf("Invalid inst\n", traceeStack);
                } else {
                        printf("Valid inst\n", traceeStack);
                }
                int wstatus;
                printf("Fork child, stack @ %p\n", traceeStack);
                int pid = fork();
                if (pid == 0) {
                        // printf("Before traceme\n");
                        ptrace(PTRACE_TRACEME, 0, 0, 0);
                        raise(SIGSTOP);
                        // printf("After traceme\n");
                        return 0;
                }
                printf("Child pid %d\n", pid);
                waitpid(pid, &wstatus, 0); //Wait for the tracee init signal
                                           // perror("waitpid");

                user_regs_struct resultState;
                user_regs_struct startState;

                iovec startIovec;
                startIovec.iov_base = (void*) &startState;
                startIovec.iov_len = sizeof(user_regs_struct);

                iovec resultIovec;
                resultIovec.iov_base = (void*) &resultState;
                resultIovec.iov_len = sizeof(user_regs_struct);

                // read back
                ret = ptrace(PTRACE_GETREGSET, pid, NT_PRSTATUS, &startIovec);
                // perror("ptrace");
                printf("Get reg set ret = %d\n", ret);
                printf("Child era %lx, changed to %p\n", startState.csr_era, instructionPointer);
                startState.csr_era = (uint64_t)instructionPointer;
                startState.csr_badv = 0;
                startState.regs[0] = 0;
                startState.regs[13] = 0;

                //set all GPRs to 0 and PC to instruction address
                ptrace(PTRACE_SETREGSET, pid, NT_PRSTATUS, &startIovec);
                // perror("ptrace");
                //write instruction
                // memcpy(data->instructionPointer, &data->currentInstruction, instructionSize);
                // clearCache(data->instructionPointer, data->instructionPointer + instructionSize);
                //Force single step and catch exception on entry
                ptrace(PTRACE_SINGLESTEP, pid, 0, 0);
                // perror("ptrace");
                waitpid(-1, &wstatus, 0);
                //Force single step and catch resulting exception of execution on exit
                ptrace(PTRACE_SINGLESTEP, pid, 0, 0);
                // perror("ptrace");
                waitpid(-1, &wstatus, 0);
                ptrace(PTRACE_GETSIGINFO, pid, 0, &sig);
                // perror("ptrace");
                printf("Signal: %d\n", sig.si_signo);
                ptrace(PTRACE_GETREGSET, pid, NT_PRSTATUS, &resultIovec); //gets resulting state
                                                                          // perror("ptrace");

                for (int i = 0;i < 32;i++) {
                        if (startState.regs[i] != resultState.regs[i]) {
                                printf("Mismatch: regs[%d]: %lx != %lx\n", i, startState.regs[i], resultState.regs[i]);
                        }
                }
                int actual_illegal = resultState.csr_era == startState.csr_era;
                if (resultState.csr_badv != 0x0) {
                        printf("BADV = %lx\n", resultState.csr_badv);
                }
                if (resultState.csr_era == startState.csr_era + 0x4) {
                        printf("Valid\n");
                } else if (resultState.csr_era == startState.csr_era) {
                        printf("Illegal instruction\n");
                } else {
                        printf("Unknown era = %lx\n", resultState.csr_era);
                }
                printf("rd = %lx\n", resultState.regs[inst & 0x1f]);
                printf("rj = %lx\n", resultState.regs[(inst >> 5) & 0x1f]);
                printf("rk = %lx\n", resultState.regs[(inst >> 10) & 0x1f]);
                assert (binutils_illegal && !actual_illegal);
                break;
                if (binutils_illegal && !actual_illegal) {
                        printf("Hidden instruction: %x\n", inst);
                        FILE *fp = fopen("mismatch.txt", "at");
                        fprintf(fp, "Mismatch: %x\n", inst);
                        fclose(fp);
                        system("objdump -b binary -m Loongarch64 -D temp");
                }
        }
        return 0;
}
