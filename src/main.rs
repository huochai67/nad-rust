use env_logger::{Env, Target};
use nad_rust::run_loop;

struct Mypipe{
    pub data : String,
}

impl Mypipe {
    fn new() -> Mypipe{
        Mypipe{data : String::new()}
    }
}

impl std::io::Write for Mypipe {
    fn write(&mut self, buf: &[u8]) -> Result<usize, std::io::Error>{
        let s = match std::str::from_utf8(buf) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        self.data += s;
        println!("{}", self.data);
        Ok(buf.len())
    }
    fn flush(&mut self) -> Result<(), std::io::Error>{
        self.data.clear();
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    //let pipe = Mypipe::new();
    env_logger::Builder::from_env(Env::default().filter_or("RUST_LOG", "info")).target(Target::Stdout).init();

    log::info!("Program Start.");

    let _ = run_loop().await;
}
