use std::error::Error;

pub struct Source {
    pub name: String,
    pub url: String,
}

impl Source {
    pub fn download_to_string(&self) -> Result<String, Box<dyn Error>> {
        let req = reqwest::get(self.url.as_str())?.text()?;
        Ok(req)
    }
}
