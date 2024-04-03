mod s_3;

fn main() {
    if let Err(e) = s_3::main() {
        eprintln!("Error running S3 main: {:?}", e);
    }
}
