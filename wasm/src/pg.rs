
// https://rust-by-example-ext.com/rand.html
extern crate rand;
use rand::thread_rng;
use rand::Rng;

const SPECIAL_CHARS: [char; 14] = ['!', '#', '$', '%', '&', '*', ']', '[', '(', ')', '{', '}', '+', '-',];

pub fn generatepwd(
    _nchars: i32, 
    _specialchars: bool,
    _uppercase: bool,
    _lowercase: bool,
    _numbers: bool) -> String {
    
    let mut _vsize = 0;

    if _specialchars == true
         { _vsize += 14 }
    if _uppercase == true
        { _vsize += 26 }
    if _lowercase == true
        { _vsize += 26 }
    if _numbers == true
        { _vsize += 10 }
    
    if _vsize == 0
        { _vsize += 26 }

    let mut chars: Vec<char> = Vec::with_capacity(_vsize);

    // Default Lowercase chars
    if _lowercase == true {    
        for i in 'a' as u8..'z' as u8 + 1 {
            chars.push(i as char);
        }
    }

    if _uppercase == true {
        for i in 'A' as u8..'Z' as u8 + 1 {
            chars.push(i as char);
        }
    }

    if _numbers == true {
        for i in '0' as u8..'9' as u8 + 1 {
            chars.push(i as char);
        }
    }

    if _specialchars == true {
        chars.append(&mut SPECIAL_CHARS.to_vec());
    }
    


    let mut rng = thread_rng();    
    let password: String = (0.._nchars).map(|_| {
        let idx = rng.gen_range(0, _vsize);
        chars[idx] as char
    }).collect();
    
    return password;
}
