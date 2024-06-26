use std::io::{BufRead, Seek, SeekFrom};

use byteorder::{BigEndian, LittleEndian, ReadBytesExt};

pub trait EasyRead: ReadBytesExt + BufRead + Seek {
    #[inline]
    fn read_u32_le(&mut self) -> Result<u32, std::io::Error> {
        self.read_u32::<LittleEndian>()
    }

    #[inline]
    fn read_u32_be(&mut self) -> Result<u32, std::io::Error> {
        self.read_u32::<BigEndian>()
    }

    fn read_cstring(&mut self) -> Result<String, std::io::Error> {
        let mut buf = Vec::new();
        let len = self.read_until(0, &mut buf)?;
        buf.resize(len - 1, 0);
        String::from_utf8(buf).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
    }

    fn read_count_string(&mut self, len: usize) -> Result<String, std::io::Error> {
        let mut buf = vec![0; len];
        self.read_exact(&mut buf)?;
        let cleaned: Vec<_> = buf.iter().take_while(|p| **p != 0).copied().collect();

        String::from_utf8(cleaned).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
    }

    #[inline]
    fn read_u16_le(&mut self) -> Result<u16, std::io::Error> {
        self.read_u16::<LittleEndian>()
    }

    fn read_at<F, T>(&mut self, seek: SeekFrom, f: F) -> Result<T, std::io::Error>
    where
        F: Fn(&mut Self) -> Result<T, std::io::Error>,
    {
        let tell = self.stream_position()?;
        self.seek(seek)?;
        let res = f(self)?;
        self.seek(SeekFrom::Start(tell))?;
        Ok(res)
    }

    fn read_bytes(&mut self, count: usize) -> Result<Vec<u8>, std::io::Error> {
        let mut buffer = vec![0; count];
        self.read_exact(&mut buffer)?;
        Ok(buffer)
    }
}

/// All types that implement `Read`, `BufRead` and `Seek` get methods defined in `EasyRead`
/// for free.
impl<R: std::io::Read + BufRead + Seek + ?Sized> EasyRead for R {}
