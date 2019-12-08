use std::io::{LineWriter, Write};
use std::fs::File;
use std::error::Error;
use std::time::Duration;
use std::io;

pub(crate) struct Writer {
    gen_times: LineWriter<File>,
    sort_times: LineWriter<File>
}

impl Writer {
    pub fn new(gen_file: &str, sort_file: &str) -> Result<Writer, Box<dyn Error>> {
        Ok(
            Writer {
                gen_times: LineWriter::new(File::create(gen_file)?),
                sort_times: LineWriter::new(File::create(sort_file)?)
            }
        )
    }
    pub fn write_gen(&mut self, length: usize, time: Duration) -> io::Result<()> {
        let line: String = format!("{},{}\n", length, time.as_nanos());
        self.gen_times.write_all(line.as_bytes())?;
        Ok(())
    }

    pub fn write_run(&mut self, algo: &str, length: usize, time: Duration) -> io::Result<()>{
        let line: String = format!("{},{},{}\n", algo, length, time.as_nanos());
        self.sort_times.write_all(line.as_bytes())?;
        Ok(())
    }
}