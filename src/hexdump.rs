use std::str;

// FIXME byte order
pub fn print_hexdump(data: &[u8]) {
    let mut c = 0;
    let mut line: [u8; 16] = [0; 16];
    for b in data {
        if (c % 16) == 0 {
            line = [0; 16];
            println!("");
            print!("{:08x}: ", c);
        }
        
        line[c % 16] = *b;
        c = c + 1;
        if (c % 16) == 0 {
            // print line
            for b in 0..8 {
                let w = ((line[2*b] as u16) << 8) + (line[2*b + 1] as u16);
                print!(" {:04x}", w);
            }
            // TODO
            // let utf8_replacement_bytes: [u8; 3] = [0xEF, 0xBF, 0xBD];
            // let utf8_replacement = String::from_utf8_lossy(&utf8_replacement_bytes);
            // let s = String::from_utf8_lossy(&line);
            // let s = s.replace(utf8_replacement, ".");
            // print!("  {}", s);
        }
    }
    if (c % 16) != 0 {
        for b in 0..((c % 16) / 2) {
            let w = ((line[2*b] as u16) << 8) + (line[2*b + 1] as u16);
            print!(" {:04x}", w);
        }
        // let s = String::from_utf8_lossy(&line);
        // print!("  {}", s);
    }
}