use std::{env, process, thread, time, net};

fn unix_now()->i128{
    time::SystemTime::now().duration_since(time::UNIX_EPOCH).expect("get_time").as_millis() as i128
}

fn main() {
    let args:Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("bad arg");
        process::exit(1);
    }
    let host = args[1].clone();
    let port = args[2].as_str();

    let socket = net::UdpSocket::bind( "0.0.0.0:".to_string() + port).expect("Bind");
    let listen = socket.try_clone().expect("clone socket");
    let mut counter = 0;
    thread::spawn(move || loop {
        let mut buf:[u8; 16] = [0;16];
        match listen.recv(&mut buf){
            Ok(16)=> {
                counter += 1;
                let received = i128::from_le_bytes(buf);
                let now = unix_now();
                let diff = now - received;
                println!("{:>6} now: {}, diff: {}ms", counter, now / 1000, diff)
            },
            Ok(other_len)=>println!("received_bytes: {}", other_len),
            Err(err)=>println!("receive error: {}", err),
        }
    });


    thread::spawn(|| {
        let mut lastTime = unix_now();
        loop{
            let n = unix_now();
            if n < lastTime {
                println!("time jump backward for {}ms", lastTime - n);
            }
            lastTime = n;
            thread::sleep(time::Duration::from_millis(10));
    }});

    loop {
        socket.connect(host.to_string() + ":"+port).expect("connect");
        socket.send(unix_now().to_le_bytes().as_ref()).expect("send time");
        thread::sleep(time::Duration::from_secs(1));
    }
}
