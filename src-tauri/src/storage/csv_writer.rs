use std::fs::{File, create_dir_all};
use std::io::{Write, BufWriter};
use std::path::{Path, PathBuf};
use chrono::Utc;

pub struct CsvWriter {
    session_path: PathBuf,
    bvp_file: Option<BufWriter<File>>,
    eda_file: Option<BufWriter<File>>,
    temp_file: Option<BufWriter<File>>,
    acc_file: Option<BufWriter<File>>,
    hr_file: Option<BufWriter<File>>,   // stub
    ibi_file: Option<BufWriter<File>>,  // stub
    session_start_timestamp: i64,
}

impl CsvWriter {
    pub fn new(base_path: &str, session_name: &str) -> Result<Self, std::io::Error> {
        let session_path = Path::new(base_path).join(session_name);
        create_dir_all(&session_path)?;

        let session_start_timestamp = Utc::now().timestamp();

        let mut writer = Self {
            session_path: session_path.clone(),
            bvp_file: None,
            eda_file: None,
            temp_file: None,
            acc_file: None,
            hr_file: None,
            ibi_file: None,
            session_start_timestamp,
        };

        writer.init_files()?;
        Ok(writer)
    }

    fn init_files(&mut self) -> Result<(), std::io::Error> {
        // BVP.csv - no header, just values
        let bvp_path = self.session_path.join("BVP.csv");
        self.bvp_file = Some(BufWriter::new(File::create(bvp_path)?));

        // EDA.csv - no header, just values
        let eda_path = self.session_path.join("EDA.csv");
        self.eda_file = Some(BufWriter::new(File::create(eda_path)?));

        // TEMP.csv - unix timestamp + sample rate header
        let temp_path = self.session_path.join("TEMP.csv");
        let mut temp_file = BufWriter::new(File::create(temp_path)?);
        writeln!(temp_file, "{}.000000", self.session_start_timestamp)?;
        writeln!(temp_file, "4.000000")?; // 4 Hz sample rate
        self.temp_file = Some(temp_file);

        // ACC.csv - sample rate header (32 Hz for all axes)
        let acc_path = self.session_path.join("ACC.csv");
        let mut acc_file = BufWriter::new(File::create(acc_path)?);
        writeln!(acc_file, "32,32,32")?;
        self.acc_file = Some(acc_file);

        // HR.csv - stub for now
        let hr_path = self.session_path.join("HR.csv");
        self.hr_file = Some(BufWriter::new(File::create(hr_path)?));

        // IBI.csv - stub for now
        let ibi_path = self.session_path.join("IBI.csv");
        self.ibi_file = Some(BufWriter::new(File::create(ibi_path)?));

        Ok(())
    }

    pub fn write_bvp(&mut self, value: f32) -> Result<(), std::io::Error> {
        if let Some(file) = &mut self.bvp_file {
            writeln!(file, "{}", value)?;
            file.flush()?;
        }
        Ok(())
    }

    pub fn write_eda(&mut self, value: f32) -> Result<(), std::io::Error> {
        if let Some(file) = &mut self.eda_file {
            writeln!(file, "{:.6}", value)?;
            file.flush()?;
        }
        Ok(())
    }

    pub fn write_temp(&mut self, value: f32) -> Result<(), std::io::Error> {
        if let Some(file) = &mut self.temp_file {
            writeln!(file, "{:.2}", value)?;
            file.flush()?;
        }
        Ok(())
    }

    pub fn write_acc(&mut self, x: f32, y: f32, z: f32) -> Result<(), std::io::Error> {
        if let Some(file) = &mut self.acc_file {
            writeln!(file, "{},{},{}", x, y, z)?;
            file.flush()?;
        }
        Ok(())
    }

    pub fn close(&mut self) -> Result<(), std::io::Error> {
        // Flush and drop all files
        if let Some(mut file) = self.bvp_file.take() {
            file.flush()?;
        }
        if let Some(mut file) = self.eda_file.take() {
            file.flush()?;
        }
        if let Some(mut file) = self.temp_file.take() {
            file.flush()?;
        }
        if let Some(mut file) = self.acc_file.take() {
            file.flush()?;
        }
        if let Some(mut file) = self.hr_file.take() {
            file.flush()?;
        }
        if let Some(mut file) = self.ibi_file.take() {
            file.flush()?;
        }
        Ok(())
    }
}