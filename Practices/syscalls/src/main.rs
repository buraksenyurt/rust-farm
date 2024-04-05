use std::ffi::CString;
use std::io;

fn main() -> io::Result<()> {
    // Example 01
    // Şu an üzerinde çalıştığımız programın OS tarafından verilen PID değerini öğrenmek için
    unsafe {
        let pid = libc::syscall(libc::SYS_getpid);
        println!("Programın PID değeri, {}", pid);
    }

    // Example 02
    // Sistemde bir dosya açıp içeriğini kaydetmek için
    let filename = "games.dat";
    let content = "Bu dosyada popüler oyun motorlarına ait bilgiler yer alıyor.!\n\n1 - Mario Boss\n2 - Pacman";

    unsafe {
        let c_filename = CString::new(filename).expect("CString::new failed");
        let f = libc::open(c_filename.as_ptr(), libc::O_WRONLY | libc::O_CREAT, 0o644);
        if f < 0 {
            panic!("Dosya açılamadı");
        }

        libc::write(f, content.as_ptr() as *const libc::c_void, content.len());
        libc::close(f);
    }

    Ok(())
}
