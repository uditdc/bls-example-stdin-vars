use blockless_sdk::*;

fn main() -> std::io::Result<()> {
    let mut buf = [0; 1024];
    let len = read_stdin(&mut buf)?;
    let str = std::str::from_utf8(&buf[..len as usize]);
    println!("stdinString: {}", str.unwrap_or_default());

    Ok(())
}
