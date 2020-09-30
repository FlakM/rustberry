use mcp3008::Mcp3008;

fn main() {
    let mut mcp3008 = Mcp3008::new("/dev/spidev0.0").unwrap();

    for i in 0..8 {
        println!("{}: {}", i, mcp3008.read_adc(i).unwrap());
    }
}
