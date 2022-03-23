use kernel::{
    chrdev,
    file,
    bindings,
    c_types::{c_ulong},
    prelude::*,
    file_operations::FileOperations,
    io_buffer::{IoBufferReader, IoBufferWriter},
    
};

module! {
    type: GpioChardev,
    name: b"gpio_chardev",
    author: b"ZechariahZheng",
    description: b"gpio chardev sample",
    license: b"GPL v2",
}

struct GpioCharFile;


impl FileOperations for GpioCharFile {
    kernel::declare_file_operations!(read, write);

    fn open(_shared: &(), _file: &file::File) -> Result {
        Ok(())
    }

    fn read(_this: (), _file: &file::File, buf: &mut impl IoBufferWriter, _:u64) -> Result<usize> {
        let s = "yes";
        let mut tmpBuf = s.as_bytes(); 
        buf.write_slice(&mut tmpBuf)?;
        let len = buf.len();

        Ok(len)
    }

    fn write(_this: (), _file: &file::File, buf: &mut impl IoBufferReader, len: u64) -> Result<usize> {
        let total_len = buf.len();
        let mut chunkbuf = [0; 256];
        while !buf.is_empty() {
            let len = chunkbuf.len().min(buf.len());
            let chunk = &mut chunkbuf[0..len];
            buf.read_slice(chunk)?;
        }
        if chunkbuf[0] == 1 {   //set GPIO2 High
            let gpset0 = unsafe {bindings::ioremap( 0x3f20001c, 4)};
            let raw_gpset0 = gpset0 as *mut c_ulong;
            unsafe {
                *raw_gpset0 = *raw_gpset0 | (0x1 << 2);
            }
            unsafe{bindings::iounmap(gpset0)};
        } else {    //set GPIO2 Low
            let gpclr0 = unsafe {bindings::ioremap( 0x3f200028, 4)};
            let raw_gpclr0 = gpclr0 as *mut c_ulong;
            unsafe {
                *raw_gpclr0 = *raw_gpclr0 | (0x1 << 2);
                
            }
            unsafe{bindings::iounmap(gpclr0)};
        }
        Ok(total_len)
    }
}
struct GpioChardev {
    _dev: Pin<Box<chrdev::Registration<1>>>,
}

impl KernelModule for GpioChardev {
    fn init(name: &'static CStr, module: &'static ThisModule) -> Result<Self> {
        pr_info!("gpio character device sample (init)\n");
        let gpfsel0 = unsafe {bindings::ioremap( 0x3f200000, 4)};
        let raw_gpfsel0 = gpfsel0 as *mut c_ulong;
        unsafe {
            *raw_gpfsel0 = *raw_gpfsel0 & ( !( 0x6 << ( 2 * 3 ) ) );
            *raw_gpfsel0 = *raw_gpfsel0 | ( 0x1 << ( 2 * 3 ) );
        }
        let mut chrdev_reg = chrdev::Registration::new_pinned(name, 0, module)?;
        chrdev_reg.as_mut().register::<GpioCharFile>()?;

        Ok(GpioChardev {
            _dev: chrdev_reg,
        })
    }
}

impl Drop for GpioChardev {
    fn drop(&mut self) {
        pr_info!("gpio character device sample (exit)\n");
    }
}