use serde::Deserialize;
use reqwest::{self,Url};
use ffi_convert::*;

#[derive(Deserialize, Debug)]
pub struct Me {
    pub basic_email_prefs: String,
    pub login: String
}

#[repr(C)]
#[derive(CReprOf, CDrop)]
#[target_type(Me)]
pub struct CMe {
    pub basic_email_prefs: *const libc::c_char,
    pub login: *const libc::c_char,
}

pub struct Api {
    pub base_url: Url,
    pub api_key: String,
}

impl Api {
    #[no_mangle]
    pub fn new(base_url: String, api_key: String) -> Api {
        let url = Url::parse(&base_url).unwrap();
        Api {
            base_url: url,
            api_key: api_key,
        }
    }

    #[no_mangle]
    pub extern fn me(&self) -> Result<Me, Box<std::error::Error>>{
        let request_url = format!("{base}/me", base=self.base_url);

        let client = reqwest::blocking::Client::new();
        let response = client
            .get(request_url)
            .header("Circle-Token", &self.api_key)
            .send()?;

        let me: Me = response.json()?;
        Ok(me)
    }
}