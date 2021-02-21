use std::env;
use kvs::KvStore;

fn set(mut kv: KvStore, args: Vec<String>) {
    panic!("unimplemented");
    /*
    if args.len() != 4 {
        panic!();
    }
    kv.set(args[2].to_owned(), args[3].to_owned());
    */
}

fn get(kv: KvStore, args: Vec<String>) {
    panic!("unimplemented");
    /*
    if args.len() != 3 {
        panic!();
    }
    kv.get(args[2].to_owned());
    */
}

fn rm(kv: KvStore, args: Vec<String>) {
    panic!("unimplemented");
    /*
    if args.len() != 3 {
        panic!();
    }
    kv.get(args[2].to_owned());
    */
}

fn version(args: Vec<String>) {
    if args.len() != 2 {
        panic!();
    }
    println!("{}", env!("CARGO_PKG_VERSION"))
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!();
    }

    if args.len() > 4 {
        panic!();
    }

    let cmd = &args[1];

    let kv = KvStore::new();
    
    match &cmd[..] {
        "-V" => version(args),
        "set" => set(kv, args),
        "get" => get(kv, args),
        "rm" => rm(kv, args),
        _ => panic!(),
    }
}
