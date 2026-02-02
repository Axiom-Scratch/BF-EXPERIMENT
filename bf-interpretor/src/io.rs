use std::io::{BufReader, BufWriter, Read, Write};

pub struct Input<R: Read> {
    reader: BufReader<R>,
    buf: [u8; 1],
}

impl<R: Read> Input<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader: BufReader::new(reader),
            buf: [0u8; 1],
        }
    }

    pub fn read_byte(&mut self) -> Result<u8, String> {
        let read = self
            .reader
            .read(&mut self.buf)
            .map_err(|e| format!("stdin read failed: {}", e))?;
        if read == 0 {
            Ok(0)
        } else {
            Ok(self.buf[0])
        }
    }
}

pub struct Output<W: Write> {
    writer: BufWriter<W>,
}

impl<W: Write> Output<W> {
    pub fn new(writer: W) -> Self {
        Self {
            writer: BufWriter::new(writer),
        }
    }

    pub fn write_byte(&mut self, byte: u8) -> Result<(), String> {
        self.writer
            .write_all(&[byte])
            .map_err(|e| format!("stdout write failed: {}", e))
    }

    pub fn flush(&mut self) -> Result<(), String> {
        self.writer
            .flush()
            .map_err(|e| format!("stdout flush failed: {}", e))
    }
}

pub struct Debug<W: Write> {
    writer: BufWriter<W>,
}

impl<W: Write> Debug<W> {
    pub fn new(writer: W) -> Self {
        Self {
            writer: BufWriter::new(writer),
        }
    }

    pub fn writer(&mut self) -> &mut BufWriter<W> {
        &mut self.writer
    }

    pub fn write_fmt(&mut self, args: std::fmt::Arguments) -> Result<(), String> {
        self.writer
            .write_fmt(args)
            .map_err(|e| format!("stderr write failed: {}", e))
    }

    pub fn flush(&mut self) -> Result<(), String> {
        self.writer
            .flush()
            .map_err(|e| format!("stderr flush failed: {}", e))
    }
}
