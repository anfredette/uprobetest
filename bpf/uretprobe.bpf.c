// Some uprobe test code
#include <linux/bpf.h>
#include <bpf/bpf_helpers.h>

SEC("uretprobe/probe")
int probe(struct pt_regs *ctx) {
  bpf_printk("In uretprobe");
  return 0;
}

char _license[] SEC("license") = "GPL";
