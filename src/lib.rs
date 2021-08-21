mod api_v1;

use crate::api_v1::Me;
use ffi_convert::CReprOf;
use crate::api_v1::CMe;
use std::ffi::CStr;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn api_v1(base_url: *const c_char, api_key: *const c_char) -> *mut api_v1::Api {
    let burl = unsafe { CStr::from_ptr(base_url) };
    let akey = unsafe { CStr::from_ptr(api_key) };

    let url = match burl.to_str() {
        Ok(s) => s,
        Err(_) => panic!("Couldn't load")
    };
    let key = match akey.to_str() {
        Ok(s) => s,
        Err(_) => panic!("Couldn't load")
    };
    Box::into_raw(Box::new(api_v1::Api::new(url.to_string(), key.to_string())))
}

#[no_mangle]
pub unsafe extern "C" fn api_v1_me(api: *const api_v1::Api) -> api_v1::CMe {
    let api = &*api;
    let res = match api.me() {
        Ok(r) => r,
        Err(_) => {
            let def_email = "none".to_string();
            let def_login = "none".to_string();
            Me{basic_email_prefs: def_email, login: def_login}

        },
    };
    match CMe::c_repr_of(res) {
        Ok(m) => m,
        Err(_) => {
            let def_email = "none".as_ptr() as *mut i8;
            let def_login = "none".as_ptr() as *mut i8;
            CMe{basic_email_prefs: def_email, login: def_login}
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::api_v1;
    use std::env;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[tokio::test]
    async fn me() {
        let api_key = match env::var("CIRCLE_TOKEN") {
            Ok(val) => val,
            Err(_) => "".to_string(),
        };

        let v1 = api_v1::Api::new( 
            "https://circleci.com/api/v1.1".to_string(), 
            api_key, 
        );
        match v1.me() {
            Ok(me) => {
                assert_eq!(me.login, "gmemstr".to_string())
            },
            Err(_) => {
                println!("Did not get expected result from endpoint!"); assert!(false)
            }
        };
    }
}
