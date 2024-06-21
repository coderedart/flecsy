use std::{
    borrow::Borrow,
    ffi::{CStr, CString},
    ops::Deref,
};

/// We are going to pass around a *lot* of strings via FFI
/// It is unergonomic to keep translating between CStrings (no utf-8 guarantee) and Strings (no null termination guarantee)
/// So, we are going to create a newtype that guarantees that the string is null-terminated AND valid utf-8
/// We also provide an [NullStr] which is similar to &str, but guarantees that it is null-terminated
/// NOTE: This implements AsRef<str>, BUT the returned str does NOT include the null-terminator. If you need the null-terminator, use as_c_str() to get a CStr
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct NullString(String);
/// This is to [NullString], what &[str] is to [String].
/// It is similar to &str, but guarantees that it is null-terminated
/// We provide AsRef<str> excluding the null-terminator, for the sake of convenience. You can simply use this as &str just like any other string.
/// NOTE: This implements AsRef<str>, BUT the returned str does NOT include the null-terminator. If you need the null-terminator, use as_c_str() to get a CStr
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NullStr(str);

impl AsRef<NullStr> for NullString {
    fn as_ref(&self) -> &NullStr {
        self.as_null_str()
    }
}
impl Deref for NullString {
    type Target = NullStr;
    fn deref(&self) -> &Self::Target {
        self.as_null_str()
    }
}
impl Default for NullString {
    fn default() -> Self {
        NullString::new("\0".to_string()).unwrap()
    }
}
impl NullString {
    /// Create a new NullString from a String
    /// If the string is not null-terminated, it will add a null-terminator
    /// If the string contains any null bytes except for the last byte, it will return itself as an error
    /// empty strings are fine
    pub fn new(mut s: String) -> Result<Self, String> {
        // if last char is not null, we add a null char
        if !s.ends_with('\0') {
            s.push('\0');
        }
        // use cstr to verify that this is a proper null-terminated string
        match CStr::from_bytes_with_nul(s.as_bytes()) {
            Ok(_) => Ok(NullString(s)),
            Err(_) => Err(s),
        }
    }
    /// Create a new NullString from a String without checking for null-termination or null bytes before the last byte
    /// This is unsafe because NullString guarantees that the string is null-terminated and valid utf-8, and if this invariant is not maintained, it can lead to UB
    pub unsafe fn new_unchecked(s: String) -> Self {
        NullString(s)
    }

    /// Create a new NullString from a CString
    /// If the string is not valid utf-8, it will return itself as an erro
    pub fn new_cstring(s: CString) -> Result<Self, CString> {
        match s.as_c_str().to_str() {
            // unsafe: we already checked for valid str and null termination is guaranteed by cstring
            Ok(_) => Ok(unsafe {
                NullString::new_unchecked(String::from_utf8_unchecked(s.into_bytes_with_nul()))
            }),
            Err(_) => Err(s),
        }
    }
    pub fn as_null_str(&self) -> &NullStr {
        // unsafe: we know that the string inside self is valid utf-8 and null-terminated, so its fine
        unsafe { NullStr::from_utf_bytes_with_null_unchecked(self.0.as_bytes()) }
    }
}
impl AsRef<str> for NullStr {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl Deref for NullStr {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}
impl Default for &NullStr {
    fn default() -> Self {
        NullStr::EMPTY
    }
}
impl Borrow<NullStr> for NullString {
    fn borrow(&self) -> &NullStr {
        self.as_null_str()
    }
}
impl ToOwned for NullStr {
    type Owned = NullString;
    fn to_owned(&self) -> Self::Owned {
        NullString(self.as_str().to_string())
    }
}
impl NullStr {
    pub const EMPTY: &'static Self = match NullStr::new_cstr(c"") {
        Ok(s) => s,
        Err(_) => unreachable!(),
    };
    /// Create a new NullStr from a byte slice
    /// The slice must be:
    /// 1. valid utf-8
    /// 2. null-terminated
    /// 3. does not contain any null bytes except for the last byte
    /// 4. not empty (if its empty, it can't have a null terminator :D )
    pub fn from_utf_bytes_with_null(s: &[u8]) -> Result<&Self, &[u8]> {
        // check for valid utf-8
        if std::str::from_utf8(s).is_err() {
            return Err(s);
        }
        // use cstr to verify that this is a proper null-terminated string and there's no stray null bytes
        // This will also verify that the slice is not empty
        if CStr::from_bytes_with_nul(s).is_err() {
            return Err(s);
        }

        // unsafe, but its okay because we know that its both valid str and valid CStr
        Ok(unsafe { Self::from_utf_bytes_with_null_unchecked(s) })
    }
    /// Create a new NullStr from a byte slice without checking for utf-8 validity or null-termination or null bytes before the last byte
    /// This is unsafe because NullStr guarantees that the string is null-terminated and valid utf-8, and if this invariant is not maintained, it can lead to UB
    pub const unsafe fn from_utf_bytes_with_null_unchecked(s: &[u8]) -> &Self {
        &*(s as *const [u8] as *const NullStr)
    }
    pub const fn new(s: &str) -> Result<&Self, &str> {
        // use cstr to verify that this is a proper null-terminated string
        match CStr::from_bytes_with_nul(s.as_bytes()) {
            // unsafe, but its okay because we know that its both valid str and valid CStr
            Ok(_) => Ok(unsafe { Self::from_utf_bytes_with_null_unchecked(s.as_bytes()) }),
            Err(_) => Err(s),
        }
    }
    pub const fn new_cstr(s: &CStr) -> Result<&Self, &CStr> {
        match s.to_str() {
            // unsafe: validated both cstr and str, so its fine
            Ok(_) => unsafe {
                Ok(Self::from_utf_bytes_with_null_unchecked(
                    s.to_bytes_with_nul(),
                ))
            },
            Err(_) => Err(s),
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0[..self.0.len() - 1]
    }
    pub fn as_ptr(&self) -> *const i8 {
        self.0.as_ptr() as *const i8
    }
    pub fn as_c_str(&self) -> &CStr {
        // unsafe: no need to check for null-termination, our invariant guarantees it :)
        unsafe { CStr::from_bytes_with_nul_unchecked(self.0.as_bytes()) }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    /// checks if null string works
    #[test]
    fn test_null_string() {
        let s = NullString::new("hello".to_string()).unwrap();
        assert_eq!(s.as_str(), "hello");
        assert_eq!(s.as_c_str(), c"hello");
    }
    /// checks if null string from str works
    #[test]
    fn test_null_str() {
        let s = NullStr::new("hello\0").unwrap();
        assert_eq!(s.as_ref(), "hello");
        assert_eq!(s.as_c_str(), c"hello");
    }
    /// checks if null string from cstring works
    #[test]
    fn test_null_string_cstring() {
        let s = CString::new("hello").unwrap();
        let s = NullString::new_cstring(s).unwrap();
        assert_eq!(s.as_str(), "hello");
        assert_eq!(s.as_c_str(), c"hello");
    }
    /// checks if null str from cstr works
    #[test]
    fn test_null_str_cstr() {
        let s = CString::new("hello").unwrap();
        let s = NullStr::new_cstr(&s).unwrap();
        assert_eq!(s.as_c_str(), c"hello");
    }
    /// test that invalid utf-8 cstrings are rejected
    #[test]
    fn test_null_string_cstring_invalid() {
        let s = CString::new(b"hello\xff".to_vec()).unwrap();
        let s = NullString::new_cstring(s);
        assert!(s.is_err());
    }
    /// test that strings with null bytes are rejected
    #[test]
    fn test_null_string_invalid() {
        let s = NullString::new("hello\0oasf".to_string());
        assert!(s.is_err());
    }
}
