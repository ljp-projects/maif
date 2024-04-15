pub mod maif {
    use std::fs::File;
    use std::io::BufReader;
    use std::io::Read;

    #[derive(Debug)]
    pub struct MAIFHeader {
        width: u8,
        height: u8,
        datetime: String,
    }

    #[derive(Debug)]
    pub struct MAIFPixel {
        tone_sign: bool,
        alpha_channel: f64,
    }

    #[derive(Debug)]
    pub struct MAIFImage {
        header: MAIFHeader,
        pixels: Vec<MAIFPixel>,
    }

    impl MAIFImage {
        pub fn from_reader(reader: BufReader<File>) -> Self {
            let mut w = 0u8;
            let mut h = 0u8;
            let mut dt: String = String::new();
            let mut byte_n = 1usize;
            let mut pxs: Vec<MAIFPixel> = Vec::new();
            let mut buffer: Vec<char> = Vec::new();

            for byte_or_error in reader.bytes() {
                let byte = byte_or_error.unwrap();
                let mut byte_str = format!("{:b}", byte);

                while byte_str.len() < 8 {
                    byte_str.insert(0, '0')
                }

                match byte_n {
                    0 => (),
                    1 => w = byte,
                    2 => h = byte,
                    3..=25 => buffer.push(byte as char),
                    26 => {
                        buffer.push(byte as char);
                        dt = buffer.iter().collect();
                        buffer.clear()
                    }
                    26.. => pxs.push(MAIFPixel {
                        tone_sign: { byte_str[0..=0] == *"1" },
                        alpha_channel: {
                            (u8::from_str_radix(&byte_str[1..], 2).unwrap() as f64) / 128.0
                        },
                    }),
                }

                byte_n += 1;
            }

            MAIFImage {
                header: MAIFHeader {
                    width: w,
                    height: h,
                    datetime: dt,
                },
                pixels: pxs,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::maif::MAIFImage;
    use std::fs::File;
    use std::io::BufReader;

    #[test]
    fn from_reader() {
        let my_buf = BufReader::new(File::open("/Users/geez/Desktop/test.maif").unwrap());
        let img = MAIFImage::from_reader(my_buf);

        println!("{:#?}", img)
    }
}
