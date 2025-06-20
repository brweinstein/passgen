use std::io;

#[cfg(target_os = "linux")]
fn get_entropy(buf: &mut [u8]) -> io::Result<()> {
    let ret = unsafe {
        libc::syscall(
            libc::SYS_getrandom,
            buf.as_mut_ptr(),
            buf.len(),
            0,
        )
    };

    if ret == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

#[cfg(target_os = "macos")]
fn get_entropy(buf: &mut [u8]) -> io::Result<()> {
    let ret = unsafe { libc::getentropy(buf.as_mut_ptr() as *mut _, buf.len()) };
    if ret == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

#[cfg(target_os = "windows")]
fn get_entropy(buf: &mut [u8]) -> io::Result<()> {
    use std::ptr::null_mut;
    use std::mem::transmute;
    type RtlGenRandomFn = unsafe extern "system" fn(*mut u8, u32) -> u8;

    const ADVAPI32: &str = "advapi32.dll\0";
    const FUNC_NAME: &str = "SystemFunction036\0";

    unsafe {
        let lib = windows_sys::Win32::System::LibraryLoader::GetModuleHandleA(ADVAPI32.as_ptr() as _);
        if lib == 0 {
            return Err(io::Error::last_os_error());
        }

        let func = windows_sys::Win32::System::LibraryLoader::GetProcAddress(lib, FUNC_NAME.as_ptr() as _);
        if func.is_null() {
            return Err(io::Error::last_os_error());
        }

        let gen_random: RtlGenRandomFn = transmute(func);
        let success = gen_random(buf.as_mut_ptr(), buf.len() as u32);
        if success == 0 {
            Err(io::Error::last_os_error())
        } else {
            Ok(())
        }
    }
}

///Cryptographically secure PRNG
pub struct SecureRng {
    key: [u64; 2],
    counter: u64,
}

impl SecureRng {
    pub fn new() -> Self {
        let mut seed = [0u8; 16];
        get_entropy(&mut seed).expect("Failed to gather entropy");

        let k0 = u64::from_le_bytes(seed[0..8].try_into().unwrap());
        let k1 = u64::from_le_bytes(seed[8..16].try_into().unwrap());

        Self {
            key: [k0, k1],
            counter: 0,
        }
    }

    fn next_u64(&mut self) -> u64 {
        self.counter = self.counter.wrapping_add(1);
        let mut x = self.key[0].wrapping_add(self.counter) ^ self.key[1];
        x ^= x.rotate_left(13);
        x = x.wrapping_mul(0x5DEECE66D);
        x ^= x.rotate_right(17);
        x
    }

    pub fn gen_range(&mut self, max: usize) -> usize {
        if max == 0 {
            return 0;
        }

        let bound = u64::MAX - (u64::MAX % max as u64);
        loop {
            let val = self.next_u64();
            if val < bound {
                return (val % max as u64) as usize;
            }
        }
    }
}

pub fn random_idx(max: usize) -> usize {
    let mut rng = SecureRng::new();
    rng.gen_range(max)
}
