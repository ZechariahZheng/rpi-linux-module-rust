
use kernel::{
    chrdev,
    file,
    prelude::*,
    file_operations::FileOperations,
    io_buffer::{IoBufferReader, IoBufferWriter},
    
};

module! {
    type: YesChardev,
    name: b"yes_chardev",
    author: b"ZechariahZheng",
    description: b"Yes chardev sample",
    license: b"GPL v2",
}

struct YesCharFile;

impl FileOperations for YesCharFile {
    kernel::declare_file_operations!(read, read_iter);

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

    // fn write(_this: (), _file: &File, buf: &mut impl IoBufferReader, _: u64) -> Result<usize> {
    //     let total
    // }
}

struct YesChardev {
    _dev: Pin<Box<chrdev::Registration<1>>>,
}

impl KernelModule for YesChardev {
    fn init(name: &'static CStr, module: &'static ThisModule) -> Result<Self> {
        pr_info!("Yes character device sample (init)\n");
        
        let mut chrdev_reg = chrdev::Registration::new_pinned(name, 0, module)?;

        chrdev_reg.as_mut().register::<YesCharFile>()?;

        Ok(YesChardev {
            _dev: chrdev_reg,
        })
    }
}

impl Drop for YesChardev {
    fn drop(&mut self) {
        pr_info!("Yes character device sample (exit)\n");
    }
}