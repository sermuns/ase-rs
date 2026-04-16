#![allow(non_upper_case_globals)]
use std::io::{self, Cursor, Read, Seek, Write};

pub mod header;
pub use self::header::*;

pub mod frame;
pub use self::frame::*;

pub mod chunk;
pub use self::chunk::*;

pub mod color;
pub mod helpers;
pub use self::color::*;

/*
https://github.com/LibreSprite/LibreSprite/blob/master/docs/files/ase.txt

ASE files use Intel (little-endian) byte order.

BYTE            An 8-bit unsigned integer value
WORD            A 16-bit unsigned integer value
DWORD           A 32-bit unsigned integer value
LONG            A 32-bit signed integer value
BYTE[n]         "n" bytes.
RECT            Four LONGs (in the order: x-pos, y-pos, width, heigth)
STRING          length=WORD (how many characters to read next)
                string=BYTE[length]
                The \0 character isn't included.
PIXEL           Pixel format:
                - In RGB images, each pixel have 4 bytes in
                  this order Red, Green, Blue, Alpha.
                - In Grayscale images, each pixel have 2 bytes
                  in the order Value, Alpha.
                - In Indexed images, each pixel uses 1 byte (the index).
*/

#[derive(Debug)]
pub struct Aseprite {
    pub header: Header,
    pub frames: Vec<Frame>,
}

impl Aseprite {
    pub fn new(header: Header, frames: Vec<Frame>) -> Self {
        Self { header, frames }
    }

    pub fn from_read<R>(read: &mut R) -> io::Result<Aseprite>
    where
        R: Read + Seek,
    {
        let header = Header::from_read(read)?;
        let mut frames = Vec::with_capacity(header.frames as usize);
        for _ in 0..header.frames {
            frames.push(Frame::from_read(read, &header)?);
        }

        Ok(Self { header, frames })
    }

    pub fn write<W>(&self, wtr: &mut W) -> io::Result<()>
    where
        W: Write + Seek,
    {
        let frames_buf = vec![];
        let mut frames_wtr = Cursor::new(frames_buf);
        for frame in &self.frames {
            frame.write(&mut frames_wtr)?;
        }
        let body_len = frames_wtr.position() as u32;
        self.header.write(wtr, body_len, self.frames.len() as u16)?;
        wtr.write(&frames_wtr.into_inner())?;
        Ok(())
    }
}
