mod api_v2;

use std::ffi::CString;
use std::ffi::CStr;
use libc::c_int;
use crate::api_v2::{Collaboration, Me, Api};
use api_v2::CCollaboration;
use crate::api_v2::CMe;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn circleci_api(base_url: *const c_char, api_key: *const c_char) -> *mut Api {
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
    Box::into_raw(Box::new(api_v2::Api::new(url.to_string(), key.to_string())))
}

#[no_mangle]
pub unsafe extern "C" fn circleci_api_me(api: *const api_v2::Api) -> CMe {
    let api = &*api;
    let res = match api.me() {
        Ok(r) => r,
        Err(_) => {
            let def_id = "none".to_string();
            let def_login = "none".to_string();
            let def_name = "none".to_string();
            Me{id: def_id, login: def_login, name: def_name}
        },
    };

    let id = CString::new(res.id).unwrap();
    let login = CString::new(res.login).unwrap();
    let name = CString::new(res.name).unwrap();

    CMe{id: id.as_ptr(), login: login.as_ptr(), name: name.as_ptr()}
}

#[no_mangle]
pub unsafe extern "C" fn circleci_api_collaborations(api: *const api_v2::Api, c_buf: *mut CCollaboration, mut c_len: c_int) {
    let api = &*api;
    let mut res = match api.collaborations() {
        Ok(r) => r,
        Err(_) => {
            let v1 = "none".to_string();
            let v2 = "none".to_string();
            let v3 = "none".to_string();
            vec![Collaboration { vcs_type: v1, name: v2, avatar_url: v3 }]
        },
    };

    let ccolabs: Vec<CCollaboration> = res.drain(1..).map(|x| {
        let vcs_type = x.vcs_type.as_ptr() as *mut i8;
        let name = x.name.as_ptr() as *mut i8;
        let avatar_url = x.avatar_url.as_ptr() as *mut i8;
        CCollaboration{vcs_type, name, avatar_url}
    }).collect();

    if c_len != (ccolabs.len() as i32) {
        c_len = ccolabs.len() as i32;
    }

    std::slice::from_raw_parts_mut(c_buf, c_len as usize)
        .copy_from_slice(&ccolabs);
}

#[cfg(test)]
mod tests {
    use crate::api_v2::{self, Collaboration};
    use std::env;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn me() {
        let api_key = match env::var("CIRCLE_TOKEN") {
            Ok(val) => val,
            Err(_) => "".to_string(),
        };

        let v2 = api_v2::Api::new( 
            "https://circleci.com/api".to_string(), 
            api_key, 
        );
        match v2.me() {
            Ok(me) => {
                assert_eq!(me.login, "gmemstr".to_string())
            },
            Err(_) => {
                println!("Did not get expected result from endpoint!"); assert!(false)
            }
        };
    }

    #[test]
    fn collaborations() {
        let api_key = match env::var("CIRCLE_TOKEN") {
            Ok(val) => val,
            Err(_) => "".to_string(),
        };

        let v2 = api_v2::Api::new( 
            "https://circleci.com/api".to_string(), 
            api_key, 
        );
        let collabs: Vec<Collaboration> = match v2.collaborations() {
            Ok(c) => c,
            Err(_) => {
                println!("Did not get expected result from endpoint!"); 
                panic!("");
            }
        };
        let mut has = false;
        for c in &collabs {
            if c.name == "gmemstr" { has = true; }
        }

        assert_eq!(has, true)
    }
}
