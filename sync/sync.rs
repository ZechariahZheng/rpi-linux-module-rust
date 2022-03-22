use kernel::prelude::*;
use kernel::{
    spinlock_init, mutex_init,
    sync::{SpinLock, Mutex},
};

module! {
    type: SyncExample,
    name: b"Sync_example",
    author: b"ZechariahZheng",
    description: b"A simple Sync example",
    license: b"GPL v2",
}

kernel::init_static_sync! {
    static SAMPLE_SPINLOCK: SpinLock<u32> = 0;
    static SAMPLE_MUTEX: Mutex<u32> = 0;
}

fn global_synchronization_example() {
    *SAMPLE_SPINLOCK.lock() = 10;
    *SAMPLE_MUTEX.lock() = 20;
    pr_info!("SAMPLE_SPINLOCK: {}\n", *SAMPLE_SPINLOCK.lock());
    pr_info!("SAMPLE_MUTEX: {}\n", *SAMPLE_MUTEX.lock());
}

struct SyncExample;

impl KernelModule for SyncExample {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        global_synchronization_example();
        //Test mutexes
        {
            // SAFETY: `init` is called below.
            let mut data = Pin::from(Box::try_new(unsafe { Mutex::new(0) })?);
            mutex_init!(data.as_mut(), "RustSync::init::data1");
            *data.lock() = 30;
            pr_info!("Value: {}\n", *data.lock())
        }
        // Test spinlocks.
        {
            // SAFETY: `init` is called below.
            let mut data = Pin::from(Box::try_new(unsafe { SpinLock::new(0) })?);
            spinlock_init!(data.as_mut(), "RustSync::init::data2");
            *data.lock() = 40;
            pr_info!("Value: {}\n", *data.lock());
        }
        Ok(SyncExample)
    }
}

impl Drop for SyncExample {
    fn drop(&mut self) {
        pr_info!("Bye world from rust!\n");
    }
}


