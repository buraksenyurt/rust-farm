use std::ffi::{CStr, CString};
use std::os::raw::c_void;
use std::{io, mem, ptr};

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

    // Example 03
    // Bellek operasyonu örneği
    unsafe {
        // 4096 byte'lık bir bellek bölgesi açılıyor
        let address = libc::mmap(
            ptr::null_mut(),
            4096,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_ANON | libc::MAP_PRIVATE,
            -1,
            0,
        );

        if address == libc::MAP_FAILED {
            panic!("Bellek ayırma işlemi sırasında hata");
        }

        // Oluşturulan bellek bölgesine yazılacak veri hazırlanır
        let data = "Game data has been created.";
        let c_data = CString::new(data).unwrap();
        let len = c_data.to_bytes_with_nul().len();

        // Veri yazma operasyonu
        ptr::copy_nonoverlapping(c_data.as_ptr(), address as *mut i8, len);

        // Yazılan veri okunur
        // let data = CString::from_raw(address as *mut i8);
        /*
           from_raw kullanımı ham işaretçiden (*mut c_char) bir CString oluşturur ve Rust'ın bellek yönetim sistemi
           bu CString nesnesinin sahibi olur. Lakin, CString nesnesi scope dışına çıktığında ya da drop edildiğinde,
           Rust otomatik olarak ilgili belleği serbest bırakmaya çalışır. Ne var ki, devam eden kısımda libc::munmap ile ayrılan belleği serbest
           bırakmaya çalıştığımızdan, aynı bellek bölgesini iki kez serbest bırakmaya çalışmış oluruz.
           Birisi Rust'ın drop mekanizması, diğeri de munmap çağrısı ile.
           Bu çalışma zamanında Segmentation Fault hatasının fırlatılmasına yol açar.
           Bu nedenle aşağıdaki kullanım tercih edilmiştir.
        */
        let data = CStr::from_ptr(address as *const i8);
        println!("{:?}", data);

        // Oluşturulan bellek alanı serbest bırakılıyor
        libc::munmap(address, 4096);
    }

    // Example 04
    // Çok basit bir echo server.
    // ping mesajına pong ile cevap dönüyor

    unsafe {
        let socket = libc::socket(libc::AF_INET, libc::SOCK_STREAM, 0);
        if socket == -1 {
            panic!("Socket can't created");
        }

        let port = 7604_u16.to_be();
        let address = libc::sockaddr_in {
            sin_family: libc::AF_INET as libc::sa_family_t,
            sin_port: port,
            sin_addr: libc::in_addr {
                s_addr: libc::INADDR_ANY.to_be(),
            },
            sin_zero: [0; 8],
        };

        if libc::bind(
            socket,
            &address as *const _ as *const libc::sockaddr,
            mem::size_of::<libc::sockaddr_in>() as libc::socklen_t,
        ) == -1
        {
            libc::close(socket);
            panic!("Bind error");
        }

        if libc::listen(socket, 5) == -1 {
            libc::close(socket);
            panic!("Listen error");
        }

        println!("Server listening on port {}", port);

        loop {
            let mut client_address = mem::zeroed::<libc::sockaddr_in>();
            let mut client_address_size = mem::size_of::<libc::sockaddr_in>() as libc::socklen_t;
            let client_socket = libc::accept(
                socket,
                &mut client_address as *mut _ as *mut libc::sockaddr,
                &mut client_address_size,
            );

            if client_socket == -1 {
                continue;
            }

            println!(
                "Client has been connected successfully: {:?}",
                client_socket
            );

            let mut buffer = [0u8; 1024];
            let number_of_bytes = libc::recv(
                client_socket,
                buffer.as_mut_ptr() as *mut c_void,
                buffer.len(),
                0,
            );

            if number_of_bytes > 0 {
                let received_data = CStr::from_ptr(buffer.as_ptr() as *const i8);
                println!("Incoming Message -> {:?}", received_data);

                if received_data.to_str().unwrap_or_default() == "ping\r\n" {
                    let response = "pong\n";
                    libc::send(
                        client_socket,
                        response.as_ptr() as *const c_void,
                        response.len(),
                        0,
                    );
                }
            }

            //Kontrol etmek lazım. Port belli süre açık kalıyor ve uygulamadan tekrardan başlatılınca hata veriyor.
            libc::close(client_socket);
            //libc::close(socket);
        }
    }
}
