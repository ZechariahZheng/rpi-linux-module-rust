obj-m += gpio_chardev.o

KDIR = /home/zzs/Documents/code/rust/linux-32

MAKE = make LLVM=1 -C $(KDIR) M=$(PWD)

all: modules

modules:
	$(MAKE) modules

fmt:
	rustfmt $$(find . -name "*.rs")

clean:
	$(MAKE) clean