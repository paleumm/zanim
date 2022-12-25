//! Rust Zanim Module
use kernel::prelude::*;
use kernel::{
    file::{flags, File, Operations},
    io_buffer::{IoBufferReader, IoBufferWriter},
    miscdev,
    sync::{smutex::Mutex, Arc, ArcBorrow},
};

module! {
    type: Zanim,
    name: "zanim",
    author: "paleumm",
    description: "My very own kernel module!",
    license: "GPL",
    params: {
        devices: u32 {
           default: 1,
           permissions: 0o644,
           description: "Number of virtual devices",
       },
   },
}

struct Zanim {
    _devs: Vec<Pin<Box<miscdev::Registration<Zanim>>>>,
}

struct Device {
    number: usize,
    contents: Mutex<Vec<u8>>,
}

impl kernel::Module for Zanim {
    fn init(_name: &'static CStr, module: &'static ThisModule) -> Result<Self> {
        let count = {
            let lock = module.kernel_param_lock();
            (*devices.read(&lock)).try_into()?
        };
        pr_info!("-----------------------------\n");
        pr_info!("initializie zanim! by paleumm\n");
        pr_info!("-----------------------------\n");

        let mut devs = Vec::try_with_capacity(count)?;
        for i in 0..count {
            let dev = Arc::try_new(Device {
                number: i,
                contents: Mutex::new(Vec::new()),
            })?;
            let reg = miscdev::Registration::new_pinned(fmt!("zanim"), dev)?;
            devs.try_push(reg)?;
        }
        Ok(Self { _devs: devs })
    }
}

#[vtable]
impl Operations for Zanim {
    type OpenData = Arc<Device>;
    type Data = Arc<Device>;

    fn open(context: &Arc<Device>, file: &File) -> Result<Arc<Device>> {
        pr_info!("File for device {} was opened\n", context.number);
        if file.flags() & flags::O_ACCMODE == flags::O_WRONLY {
            context.contents.lock().clear();
        }
        Ok(context.clone())
    }

    fn read(
        data: ArcBorrow<'_, Device>,
        _file: &File,
        writer: &mut impl IoBufferWriter,
        offset: u64,
    ) -> Result<usize> {
        pr_info!("File for device {} was read\n", data.number);
        let offset = offset.try_into()?;
        let vec = data.contents.lock();
        let len = core::cmp::min(writer.len(), vec.len().saturating_sub(offset));
        writer.write_slice(&vec[offset..][..len])?;
        Ok(len)
    }

    fn write(
        data: ArcBorrow<'_, Device>,
        _file: &File,
        reader: &mut impl IoBufferReader,
        offset: u64,
    ) -> Result<usize> {
        pr_info!("File for device {} was written\n", data.number);
        let offset = offset.try_into()?;
        let len = reader.len();
        let new_len = len.checked_add(offset).ok_or(EINVAL)?;
        let mut vec = data.contents.lock();
        if new_len > vec.len() {
            vec.try_resize(new_len, 0)?;
        }
        reader.read_slice(&mut vec[offset..][..len])?;
        Ok(len)
    }
}