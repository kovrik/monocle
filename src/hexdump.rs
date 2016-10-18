// FIXME byte orde
pub fn print_hexdump(data: &[u8]) {
    let mut c = 0;
    let mut word: u16 = 0;
    for b in data {
        if (c % 16) == 0 {
            println!("");
            print!("{:08x}: ", c);
        }
        c = c + 1;
        if (c % 2) == 0 {
            print!("{:04x} ", word+(*b as u16));
        } else {
            word = (*b as u16) << 8;
        }
    }
    if (&data.len() % 2) != 0 {
        print!("{:04x} ", word);
    }
}