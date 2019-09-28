use std::error::Error;

pub trait Download {
    fn download_to_string(&self) -> Result<String, Box<dyn Error>>;
}

pub struct Source {
    pub name: String,
    pub url: String,
}

impl Download for Source {
    fn download_to_string(&self) -> Result<String, Box<dyn Error>> {
        let req = reqwest::get(self.url.as_str())?.text()?;
        return Ok(req);
    }
}
