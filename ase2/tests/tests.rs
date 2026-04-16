use ase2::Aseprite;
use rstest::rstest;
use std::{
    io::{Cursor, Read, Seek, SeekFrom},
    path::PathBuf,
};

#[rstest]
fn test_read(#[files("examples/*.aseprite")] path: PathBuf) -> std::io::Result<()> {
    let mut file = std::fs::File::open(path)?;
    let _ = ase2::Aseprite::from_read(&mut file)?;
    Ok(())
}

#[rstest]
fn test_rw(#[files("examples/*.aseprite")] path: PathBuf) -> std::io::Result<()> {
    let mut file = std::fs::File::open(path)?;
    let mut file_buf = vec![];
    file.seek(SeekFrom::Start(0))?;
    file.read_to_end(&mut file_buf)?;
    file.seek(SeekFrom::Start(0))?;
    let ase = Aseprite::from_read(&mut file)?;

    let my_buf = vec![];
    let mut wtr = Cursor::new(my_buf);
    ase.write(&mut wtr)?;
    let my_buf = wtr.into_inner();
    assert_eq!(my_buf, file_buf);

    Ok(())
}
