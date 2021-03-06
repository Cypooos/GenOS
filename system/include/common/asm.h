// stop interrupts
#define cli asm("cli"::)

// activate interrupts
#define sti asm("sti"::)

//write data to a port
#define outb(port,value) \
        asm volatile ("outb %%al, %%dx" :: "d" (port), "a" (value));

//write data and wait
#define outbp(port,value) \
        asm volatile ("outb %%al, %%dx; jmp 1f; 1:" :: "d" (port), "a" (value));

// read data on port
#define inb(port) ({    \
        unsigned char _v;       \
        asm volatile ("inb %%dx, %%al" : "=a" (_v) : "d" (port)); \
        _v;     \
})
