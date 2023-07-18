// Some uprobed test code for a simple hello world program
#include <linux/bpf.h>
#include <stdio.h>
#include <bpf/bpf_helpers.h>
//#include <linux/ptrace.h>

SEC("uprobe/hello")
int hello_probe(struct pt_regs *ctx) {
  bpf_printk("In hello uprobe");
  return 0;
}

char _license[] SEC("license") = "GPL";
