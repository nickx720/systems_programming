## Stanford Classes Notes

Currently 01
`https://reberhardt.com/cs110l/spring-2020/`
`https://youtu.be/Gr5D4Q8wmds?t=1506`
---

**What happens in C when you flood the Buffer aka *Buffer Overflow*?**

It overwrites the return address and corrupts the data stored on the stack.

---

**What are the problems with GC?**

- Disruptive
- Expensive
- Non deterministic
- Precludes manual optimization

---

**Why is fork bad?**

There could be zombie process, and it allocates memory. Need to call *wait_pid* to remove the allocation
There could be race conditions.
Nested processes.
Allocating memory in child process using malloc could corrupt data structure.

---
