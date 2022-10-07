CC = gcc
CFLAGS = -g
obj-m += memoryDriver.o
.PHONY: all clean insmod

all: userMemory
	make -C /lib/modules/$(shell uname -r)/build M=$(shell pwd) modules
	@sudo insmod memoryDriver.ko
	@sudo ./userMemory
	@sudo rmmod memoryDriver

clean:
	make -C /lib/modules/$(shell uname -r)/build M=$(shell pwd) clean


userMemory: user.o
	$(CC) -o userMemory user.o
