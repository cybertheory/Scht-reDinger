use consts::PAGE_BYTES;
use log::{ debug, info };
use std::fs::File;
use std::io::{ Error as IOError, Write };
use std::result::Result;
use std::vec;

use crate::Page::constants::HEADER_BYTES;
use crate::Page::db_header::DatabaseHeader;

mod consts;
mod header;

type PResult<T> = Result<T, IOError>;

pub struct Config {
    pub main_db_path: String,
}

pub struct Page {
    main_db_file: File,
}

impl Page {
    pub fn new(config: Config) -> PResult<Self> {
        debug!("Trying to open file {}, or create if it doesn't exist", &config.main_db_path);

        let main_db_file = File::options()
            .read(true)
            .write(true)
            .create(true)
            .open(&config.main_db_path)?;

        info!("Database file by path {} is opened successfully", &config.main_db_path);

        let mut Page = Page { main_db_file };

        Page.initialize_db_if_new()?;

        Ok(Page)
    }

    fn initialize_db_if_new(&mut self) -> PResult<()> {
        let file_metadata = self.main_db_file.metadata()?;

        let file_length = file_metadata.len();

        debug!("Checking the file metadata to create the database header if it is absent");
        if file_length == 0 {
            
            debug!("Database file is empty – creating the metapage with header");
            let mut metapage_buffer = new_page_buffer();

            let header = DatabaseHeader::default();

            // We control the input of the encoded value, we may ignore possible errors (unwrap())
            let encoded_database_header = bincode
                ::encode_into_slice(header, &mut metapage_buffer, bincode::config::standard())
                .unwrap();

            // We want to make sure that once we add new fields to the header it will not exceed
            // the preserved space, otherwise we will start overwriting the first page metadata
            assert!(encoded_database_header <= DATABASE_HEADER_BYTES);

            self.main_db_file.write(&metapage_buffer)?;

            info!("Database header has been written to metapage");
        } else {
            info!("Database file is not empty, its size is {}", file_length);
        }
        Ok(())
    }

}