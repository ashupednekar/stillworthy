// src/bpf/syscall_counter.h
#ifndef SYSCALL_COUNTER_H
#define SYSCALL_COUNTER_H

struct event {
    u32 pid;
    u64 count;
};

#endif
