use std::{env, net::{IpAddr, TcpStream}, str::FromStr, process, sync::mpsc::{Sender,channel}, thread, io::{self, Write}};

const MAX: u16 = 65535;

struct Arguments {
    flag: String,
    ipaddr: IpAddr,
    threads: u16,
}

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments");
        }
        if args.len() > 4 {
            return Err("too many args");
        }

        let f = args[1].clone();

        if let Ok(ipaddr) = IpAddr::from_str(&f) {
            return Ok(Arguments {
                flag: String::from(""),
                ipaddr,
                threads: 4,
            });
        } else {
            if f.contains("-h") || f.contains("-help") && args.len() == 2 {
                println!("Usage -j to select how many threads you want \n\n -h or -help to see the comnands");
                return Err("help");
            } else if f.contains("-h") || f.contains("-help") {
                return Err("too many arguments");
            } else if f.contains("-j") {
                let ipaddr = match IpAddr::from_str(&args[3]) {
                    Ok(s) => s,
                    Err(_) => return Err("Not a valid IP4 or IP6"),
                };
                let threads = match args[2].parse::<u16>() {
                    Ok(s) => s,
                    Err(_) => return Err("failed to parse thread number")
                };
                return Ok(Arguments{threads,flag :f,ipaddr});
            } else {
                return Err("Invalid Syntax");
            }
        }
    }
}

fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
    let mut port: u16 = start_port + 1;
    loop {
        match TcpStream::connect((addr,port)) {
            Ok(_) => {
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(_) => {}
        }

        if(MAX - port) < num_threads {
            break;
        }
        port += num_threads;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let arguments = Arguments::new(&args).unwrap_or_else(
        |err| {
            if err.contains("help") {
                process::exit(0);
            } else {
                eprintln!("{} problem pasring argumensts: {}", program, err);
                process::exit(0);
            }
        }
    );

    let num_threads = arguments.threads;
    let (tx,rx) = channel();

    for i in 0..num_threads {
        let tx = tx.clone();

        thread::spawn(move || {
            scan(tx,i,arguments.ipaddr,num_threads)
        });
    }

    let mut out = vec![];
    drop(tx);

    for p in rx {
        out.push(p);
    }
    println!("");   
    out.sort();

    for v in out {
        println!("{} is open", v);
    }

}
