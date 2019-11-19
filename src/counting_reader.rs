use std::io::{self, BufRead, Read};

pub struct CountingReader<R> {
    reader: R,
    bytes_read: u64,
}

impl<R: BufRead> From<R> for CountingReader<R> {
    fn from(reader: R) -> CountingReader<R> {
        CountingReader {
            reader,
            bytes_read: 0,
        }
    }
}

impl<R> CountingReader<R> {
    pub fn bytes_read(&self) -> u64 {
        self.bytes_read
    }
}

impl<R: Read> Read for CountingReader<R> {
    // TODO: Vectored I/O doesn't pass thru
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.reader
            .read(buf)
            .map(|cnt| {
                self.bytes_read += cnt as u64;
                cnt
            })
    }
}

impl<R: BufRead> BufRead for CountingReader<R> {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        self.reader.fill_buf()
    }
    fn consume(&mut self, amt: usize) {
        self.reader.consume(amt)
    }
}
