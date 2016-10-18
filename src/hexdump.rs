// FIXME byte order

// pub fn print_hexdump(data: &[u8]) {
//     let mut c = 0;
//     let mut word: u16 = 0;
//     for b in data {
//         if (c % 16) == 0 {
//             println!("");
//             print!("{:08x}: ", c);
//         }
//         c = c + 1;
//         if (c % 2) == 0 {
//             print!("{:04x} ", word + (*b as u16));
//         } else {
//             word = (*b as u16) << 8;
//         }
//     }
//     if (&data.len() % 2) != 0 {
//         print!("{:04x} ", word);
//     }
// }

pub fn print_hexdump(data: &[u8]) {
    let mut c = 0;
    let mut line: [u16; 16] = [0; 16];
    for b in data {
        if (c % 16) == 0 {
            line = [0; 16];
            println!("");
            print!("{:08x}: ", c);
        }
        
        line[c % 16] = *b as u16;
        c = c + 1;
        if (c % 16) == 0 {
            // print line
            for b in 0..8 {
                let w = ((line[2*b] as u16) << 8) + line[2*b + 1];
                print!(" {:04x}", w);
            }
        }
    }
    if (c % 16) != 0 {
        for b in 0..8 {
            let w = ((line[2*b] as u16) << 8) + line[2*b + 1];
            print!(" {:04x}", w);
        }
    }
}