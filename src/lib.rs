mod api_v2;

use std::ffi::CString;
use std::ffi::CStr;
use libc::c_int;
use crate::api_v2::{Collaboration, Me, Api};
use api_v2::CCollaboration;
use crate::api_v2::CMe;
use std::os::raw::c_char;
use std::{ptr, mem};

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
pub unsafe extern "C" fn circleci_api_me(api: *const api_v2::Api) -> *mut CMe {
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

    let id = CString::new(res.id).expect("Err: CString::new()").into_raw();
    let login = CString::new(res.login).expect("Err: CString::new()").into_raw();
    let name = CString::new(res.name).expect("Err: CString::new()").into_raw();

    Box::into_raw(Box::new(CMe{id, login, name}))
}

#[no_mangle]
pub unsafe extern "C" fn circleci_api_collaborations(api: *const api_v2::Api, outlen: *mut c_int) -> *mut CCollaboration {
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

    let mut ccolabs: Vec<CCollaboration> = res.drain(1..).map(|x| {
        let vcs_type = CString::new(x.vcs_type).expect("Err: CString::new()").into_raw();
        let name = CString::new(x.name).expect("Err: CString::new()").into_raw();
        let avatar_url = CString::new(x.avatar_url).expect("Err: CString::new()").into_raw();
        CCollaboration{vcs_type, name, avatar_url}
    }).collect();

    ccolabs.shrink_to_fit();
    assert!(ccolabs.len() == ccolabs.capacity());

    let len = ccolabs.len();
    let ptr = ccolabs.as_mut_ptr();
    mem::forget(ccolabs);
    ptr::write(outlen, len as c_int);
    ptr
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
