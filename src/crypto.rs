use std::io::{self, Read, Write};

pub struct XorWriter<W: Write> {
    inner: W,
    password: Vec<u8>,
    pass_idx: usize,
}

impl<W: Write> XorWriter<W> {
    pub fn new(inner: W, password: Option<&str>) -> Self {
        Self {
            inner,
            password: password.map(|s| s.as_bytes().to_vec()).unwrap_or_default(),
            pass_idx: 0,
        }
    }
}

impl<W: Write> Write for XorWriter<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if self.password.is_empty() {
            return self.inner.write(buf);
        }

        let mut enc_buf = buf.to_vec();
        let pass_len = self.password.len();
        for b in &mut enc_buf {
            *b ^= self.password[self.pass_idx % pass_len];
            self.pass_idx += 1;
        }

        self.inner.write_all(&enc_buf)?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

pub struct XorReader<R: Read> {
    inner: R,
    password: Vec<u8>,
    pass_idx: usize,
}

impl<R: Read> XorReader<R> {
    pub fn new(inner: R, password: Option<&str>) -> Self {
        Self {
            inner,
            password: password.map(|s| s.as_bytes().to_vec()).unwrap_or_default(),
            pass_idx: 0,
        }
    }
}

impl<R: Read> Read for XorReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let bytes_read = self.inner.read(buf)?;
        if bytes_read > 0 && !self.password.is_empty() {
            let pass_len = self.password.len();
            for i in 0..bytes_read {
                buf[i] ^= self.password[self.pass_idx % pass_len];
                self.pass_idx += 1;
            }
        }
        Ok(bytes_read)
    }
}
