use std::{fs::File, io::Read, path::Path};

const READBUF_SIZE: usize = 1;

fn convert_str(readbuf: &[u8; READBUF_SIZE], bytes_read: usize) -> String
{
    let mut result: String = String::new();

    for i in 0..bytes_read
    {
        let byte_str: String = format!("{:02x}", readbuf[i]);
        result.push_str(&byte_str);
    }
    return result;
}

pub fn read_file(mut file: &File, file_path: &Path) -> String
{
    let mut readbuf: [u8; READBUF_SIZE] = [0; READBUF_SIZE];
    let mut result = String::new();

    loop
    {
        let bytes_read: usize = match file.read(&mut readbuf)
        {
            Err(why) => panic!("Couldn't read {}: {}", file_path.display(), why),
            Ok(bytes_read) => bytes_read
        };

        if bytes_read == 0 {break;}
        else {result.push_str(&(convert_str(&readbuf, bytes_read)));}
    }

    return result;
}
