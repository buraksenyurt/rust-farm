# SysCall Mevzusu

İşletim sistemleri donanımlarla konuşurken sistem çağrıları gerçekleştirir ve bunlar Syscall olarak adlandırılır. Her işletim sisteminde farklılık gösterebilen bu komut setleri ile doğrudan unsafe operasyonlar icra edilmesi mümkündür. IO, network, thread, memory management işlemleri vs yapılabilir. Rust gibi sistem programlamaya yakın dillerde de işletim sisteminin çekirdeğinden Syscall talepleri gerçekleştirilebilir. Syscall'lar genellikle C, C++ ve hatta gerektiğinde Assembler gibi dillerle geliştirilir.

Bir ubuntu sistemde ls komutunun arkasında dönen dolaplara baktığımızda birçok Syscall olduğunu görürüz.

```bash
strace ls
```
Bu çağrı benim sistemimde aşağıdakine benzer bir sürece sahip.

```text
execve("/usr/bin/ls", ["ls"], 0x7ffc344ce7f0 /* 69 vars */) = 0
brk(NULL)                               = 0x5f017958b000
arch_prctl(0x3001 /* ARCH_??? */, 0x7ffc8d7dc380) = -1 EINVAL (Invalid argument)
mmap(NULL, 8192, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0x776eca148000
access("/etc/ld.so.preload", R_OK)      = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/etc/ld.so.cache", O_RDONLY|O_CLOEXEC) = 3
newfstatat(3, "", {st_mode=S_IFREG|0644, st_size=91883, ...}, AT_EMPTY_PATH) = 0
mmap(NULL, 91883, PROT_READ, MAP_PRIVATE, 3, 0) = 0x776eca131000
close(3)                                = 0
openat(AT_FDCWD, "/lib/x86_64-linux-gnu/libselinux.so.1", O_RDONLY|O_CLOEXEC) = 3
read(3, "\177ELF\2\1\1\0\0\0\0\0\0\0\0\0\3\0>\0\1\0\0\0\0\0\0\0\0\0\0\0"..., 832) = 832
newfstatat(3, "", {st_mode=S_IFREG|0644, st_size=166280, ...}, AT_EMPTY_PATH) = 0
mmap(NULL, 177672, PROT_READ, MAP_PRIVATE|MAP_DENYWRITE, 3, 0) = 0x776eca105000
mprotect(0x776eca10b000, 139264, PROT_NONE) = 0
mmap(0x776eca10b000, 106496, PROT_READ|PROT_EXEC, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x6000) = 0x776eca10b000
mmap(0x776eca125000, 28672, PROT_READ, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x20000) = 0x776eca125000
mmap(0x776eca12d000, 8192, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x27000) = 0x776eca12d000
mmap(0x776eca12f000, 5640, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_FIXED|MAP_ANONYMOUS, -1, 0) = 0x776eca12f000
close(3)                                = 0
openat(AT_FDCWD, "/lib/x86_64-linux-gnu/libc.so.6", O_RDONLY|O_CLOEXEC) = 3
read(3, "\177ELF\2\1\1\3\0\0\0\0\0\0\0\0\3\0>\0\1\0\0\0P\237\2\0\0\0\0\0"..., 832) = 832
pread64(3, "\6\0\0\0\4\0\0\0@\0\0\0\0\0\0\0@\0\0\0\0\0\0\0@\0\0\0\0\0\0\0"..., 784, 64) = 784
pread64(3, "\4\0\0\0 \0\0\0\5\0\0\0GNU\0\2\0\0\300\4\0\0\0\3\0\0\0\0\0\0\0"..., 48, 848) = 48
pread64(3, "\4\0\0\0\24\0\0\0\3\0\0\0GNU\0\302\211\332Pq\2439\235\350\223\322\257\201\326\243\f"..., 68, 896) = 68
newfstatat(3, "", {st_mode=S_IFREG|0755, st_size=2220400, ...}, AT_EMPTY_PATH) = 0
pread64(3, "\6\0\0\0\4\0\0\0@\0\0\0\0\0\0\0@\0\0\0\0\0\0\0@\0\0\0\0\0\0\0"..., 784, 64) = 784
mmap(NULL, 2264656, PROT_READ, MAP_PRIVATE|MAP_DENYWRITE, 3, 0) = 0x776ec9e00000
mprotect(0x776ec9e28000, 2023424, PROT_NONE) = 0
mmap(0x776ec9e28000, 1658880, PROT_READ|PROT_EXEC, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x28000) = 0x776ec9e28000
mmap(0x776ec9fbd000, 360448, PROT_READ, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x1bd000) = 0x776ec9fbd000
mmap(0x776eca016000, 24576, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x215000) = 0x776eca016000
mmap(0x776eca01c000, 52816, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_FIXED|MAP_ANONYMOUS, -1, 0) = 0x776eca01c000
close(3)                                = 0
openat(AT_FDCWD, "/lib/x86_64-linux-gnu/libpcre2-8.so.0", O_RDONLY|O_CLOEXEC) = 3
read(3, "\177ELF\2\1\1\0\0\0\0\0\0\0\0\0\3\0>\0\1\0\0\0\0\0\0\0\0\0\0\0"..., 832) = 832
newfstatat(3, "", {st_mode=S_IFREG|0644, st_size=613064, ...}, AT_EMPTY_PATH) = 0
mmap(NULL, 615184, PROT_READ, MAP_PRIVATE|MAP_DENYWRITE, 3, 0) = 0x776eca06e000
mmap(0x776eca070000, 438272, PROT_READ|PROT_EXEC, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x2000) = 0x776eca070000
mmap(0x776eca0db000, 163840, PROT_READ, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x6d000) = 0x776eca0db000
mmap(0x776eca103000, 8192, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x94000) = 0x776eca103000
close(3)                                = 0
mmap(NULL, 12288, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0x776eca06b000
arch_prctl(ARCH_SET_FS, 0x776eca06b800) = 0
set_tid_address(0x776eca06bad0)         = 9540
set_robust_list(0x776eca06bae0, 24)     = 0
rseq(0x776eca06c1a0, 0x20, 0, 0x53053053) = 0
mprotect(0x776eca016000, 16384, PROT_READ) = 0
mprotect(0x776eca103000, 4096, PROT_READ) = 0
mprotect(0x776eca12d000, 4096, PROT_READ) = 0
mprotect(0x5f0177aa7000, 8192, PROT_READ) = 0
mprotect(0x776eca182000, 8192, PROT_READ) = 0
prlimit64(0, RLIMIT_STACK, NULL, {rlim_cur=8192*1024, rlim_max=RLIM64_INFINITY}) = 0
munmap(0x776eca131000, 91883)           = 0
statfs("/sys/fs/selinux", 0x7ffc8d7dc3c0) = -1 ENOENT (No such file or directory)
statfs("/selinux", 0x7ffc8d7dc3c0)      = -1 ENOENT (No such file or directory)
getrandom("\x48\x3f\x5c\x19\xea\x24\xe5\x60", 8, GRND_NONBLOCK) = 8
brk(NULL)                               = 0x5f017958b000
brk(0x5f01795ac000)                     = 0x5f01795ac000
openat(AT_FDCWD, "/proc/filesystems", O_RDONLY|O_CLOEXEC) = 3
newfstatat(3, "", {st_mode=S_IFREG|0444, st_size=0, ...}, AT_EMPTY_PATH) = 0
read(3, "nodev\tsysfs\nnodev\ttmpfs\nnodev\tbd"..., 1024) = 407
read(3, "", 1024)                       = 0
close(3)                                = 0
access("/etc/selinux/config", F_OK)     = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/usr/lib/locale/locale-archive", O_RDONLY|O_CLOEXEC) = 3
newfstatat(3, "", {st_mode=S_IFREG|0644, st_size=8524864, ...}, AT_EMPTY_PATH) = 0
mmap(NULL, 8524864, PROT_READ, MAP_PRIVATE, 3, 0) = 0x776ec9400000
close(3)                                = 0
ioctl(1, TCGETS, {B38400 opost isig icanon echo ...}) = 0
ioctl(1, TIOCGWINSZ, {ws_row=24, ws_col=132, ws_xpixel=0, ws_ypixel=0}) = 0
openat(AT_FDCWD, ".", O_RDONLY|O_NONBLOCK|O_CLOEXEC|O_DIRECTORY) = 3
newfstatat(3, "", {st_mode=S_IFDIR|0755, st_size=4096, ...}, AT_EMPTY_PATH) = 0
getdents64(3, 0x5f0179592ab0 /* 69 entries */, 32768) = 2224
getdents64(3, 0x5f0179592ab0 /* 0 entries */, 32768) = 0
close(3)                                = 0
newfstatat(1, "", {st_mode=S_IFCHR|0620, st_rdev=makedev(0x88, 0x2), ...}, AT_EMPTY_PATH) = 0
write(1, "databases  development\tDownloads"..., 101databases  development	Downloads  Godot	   Library    Microsoft  obs_records  Public	     snap	Unity
) = 101
write(1, "Desktop    Documents\tgo\t   Heads"..., 106Desktop    Documents	go	   HeadsetControl  maelstrom  Music	 Pictures     RiderProjects  Templates	Videos
) = 106
close(1)                                = 0
close(2)                                = 0
exit_group(0)                           = ?
+++ exited with 0 +++
```

Tabii buradaki çağrıları anlamak için Linux Kernel ve kaynak kodları hakkında da fikir sahibi olmak lazım. Ubuntu sistemi için aşağıdaki syscall'lardan bahsedebiliriz.

## IO İşlemler
- open: Dosya açar.
- read: Dosyadan veri okur.
- write: Dosyaya veri yazar.
- close: Dosya kapatma için.
- stat, fstat: Dosya istatistiklerini ve durum bilgilerini alır.
- lseek: Dosya içindeki okuma/yazma konumunu değiştirir.

## Process Yönetimi
- fork: Çağıran işlemi klonlar.
- exit: İşlemi sonlandırır.
- waitpid: İşlemin sonlanmasını bekler.
- execve: Bir işlem içinde yeni bir program başlatır.

## Bellek Yönetimi
- mmap: Bellekte yeni bir alan ayırır.
- munmap: mmap ile ayrılan bir bellek alanını serbest bırakmak için.
- brk, sbrk: Heap alanını yönetmek için.

## IPC (InterProcess Communication)
- pipe: İki yönlü iletişim için bir kanal (pipe) oluşturur.
- shmget, shmat, shmctl: Paylaşımlı bellek segmentleri ile çalışır.
- semget, semop, semctl: Semaforlar ile çalışır.
- msgget, msgsnd, msgrcv: Mesaj kuyrukları ile çalışır.

## Network Operasyonları
- socket: Bir ağ soketi oluşturmak için.
- bind: Bir soketi belirli bir adres ile ilişkilendirmek için.
- listen, accept: Gelen bağlantıları dinleyip talepleri kabul etmek için.
- connect: Bir soketi uzaktaki bir adrese bağlamak için.
- send, recv: Ağ üzerinden veri gönderip almak için.

## Sistem Bilgisi ve Kontrolü
- getpid: Çağıran işlemin işlem kimliğini (PID) alır.
- getuid, getgid: Çağıran kullanıcının kullanıcı ve grup kimliklerini alır.
- sysinfo: Sistem hakkında genel bilgileri alır.

Buradaki bazı komutların ls içerisinde çağırıldığına dikkat edelim.