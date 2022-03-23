## Writing Linux Kernel Module in Rust for Raspberry Pi

 A simple linux kernel module in rust for raspberry pi. Those codes are based on [rust for linux](https://github.com/Rust-for-Linux/linux.git).



## General instructions

clone this repository and rust-for-linux submodule.

```shell
git clone https://github.com/ZechariahZheng/rpi-linux-module-rust.git
cd rpi-linux-module-rust
git submodule add https://github.com/Rust-for-Linux/linux.git
```

Follow this [quickstart guide](https://github.com/Rust-for-Linux/linux/blob/rust/Documentation/rust/quick-start.rst) to get started on building requirements.

Make sure that llvm and clang installed.



## Building kernel

For Raspberry Pi 2, 3, 3+ and Zero 2 W, and Raspberry Pi Compute Modules 3 and 3+ default 32-bit build configuration.

```shell
cd linux	#cd rust-for-linux
cp ../arm_defconfig arch/arm/configs
make LLVM=1 LLVM_IAS=1 ARCH=arm CROSS_COMPILE=arm-linux-gnueabihf- arm_defconfig
make LLVM=1 LLVM_IAS=1 ARCH=arm CROSS_COMPILE=arm-linux-gnueabihf- zImage modules dtbs -j8
```

For Raspberry Pi 3, 3+, 4, 400 and Zero 2 W, and Raspberry Pi Compute Modules 3, 3+ and 4 default 64-bit build configuration.

```shell
cd linux	#cd rust-for-linux
cp ../aarch64_defconfig arch/arm64/configs
make LLVM=1 LLVM_IAS=1 ARCH=arm64 CROSS_COMPILE=aarch64-linux-gnu- aarch64_defconfig
make LLVM=1 LLVM_IAS=1 ARCH=arm64 CROSS_COMPILE=aarch64-linux-gnu- Image modules dtbs -j8
```

Then flash your newly compiled kernel onto your SD Card.

```shell
sudo mkdir /mnt/fat32
sudo mkdir /mnt/ext4
sudo mount /dev/sdb1 /mnt/fat32
sudo mount /dev/sdb2 /mnt/ext4

sudo make LLVM=1 LLVM_IAS=1 ARCH=arm64 CROSS_COMPILE=aarch64-linux-gnu- INSTALL_MOD_PATH=/mnt/ext4 modules_install			    #for 64-bit
```

For 32-bit:

```shell
sudo make LLVM=1 LLVM_IAS=1 ARCH=arm CROSS_COMPILE=arm-linux-gnueabihf- INSTALL_MOD_PATH=/mnt/ext4 modules_install			 
sudo cp arch/arm/boot/zImage /mnt/fat32/kernel7.img
sudo cp arch/arm/boot/dts/bcm2837-rpi-3-b-plus.dtb /mnt/fat32/bcm2710-rpi-3-b-plus.dtb		#for rpi-3-b-plus
```

For 64-bit:

```shell
sudo make LLVM=1 LLVM_IAS=1 ARCH=arm64 CROSS_COMPILE=aarch64-linux-gnu- INSTALL_MOD_PATH=/mnt/ext4 modules_install
sudo cp arch/arm64/boot/Image /mnt/fat32/kernel8.img
sudo cp arch/arm64/boot/dts/broadcom/bcm2837-rpi-3-b-plus.dtb /mnt/fat32/bcm2710-rpi-3-b-plus.dtb
```



## Building Modules

```shell
cd moduels_name && make LLVM=1 LLVM_IAS=1 ARCH=arm CROSS_COMPILE=arm-linux-gnueabihf- 			#32-bit
cd moduels_name && make LLVM=1 LLVM_IAS=1 ARCH=arm64 CROSS_COMPILE=aarch64-linux-gnu-				#64-bit
cd hello && make LLVM=1 LLVM_IAS=1 ARCH=arm CROSS_COMPILE=arm-linux-gnueabihf- 	#for example
```



## Load and Test

Examples is tested on Raspberry Pi 3b plus(Linux kernel5.17.0-rc6-v7+).

#### hello_world

The simplest kernel module. It just prints "Hello world from rust" and "Bye world from rust".

```shell
sudo insmod hello_world.ko
sudo rmmod hello_world
dmesg	#dump kernel message
```

#### yes_chardev

A simple character device which is similar to the `yes` Unix command.

```shell
sudo insmod yes_chardev.ko
cat /proc/devices # find the major number of the device 'yes', for example, 243
sudo mknod /dev/yes c 243 0 # make a filesystem node (replace 243 with your own major number)
sudo cat /dev/yes # read from the device
sudo rmmod yes_chardev
```

#### gpio_chardev

A simple character device to control the GPIO2 on the raspberry pi.

```shell
sudo insmod gpio_chardev.ko
cat /proc/devices # find the major number of the device 'gpio', for example, 243
sudo mknod /dev/yes c 243 0 # make a filesystem node (replace 243 with your own major number)
sudo ./gpio_test on			#set GPIO High
sudo ./gpio_test off		#set GPIO Low
sudo rmmod gpio_chardev
```

#### sync

A simple example to illustrate the use of `Spinlock` and `Mutex`.

```shell
sudo insmod sync.ko
sudo rmmod sync
dmesg	#dump kernel message
```



More samples. take a look at rust-for-linux source code of the samples at `sample/rust/`.



## Reference

- `lizhuohua/linux-kernel-module-rust`https://github.com/lizhuohua/linux-kernel-module-rust.git

- `Rust-for-Linux/linux`https://github.com/Rust-for-Linux/linux.git