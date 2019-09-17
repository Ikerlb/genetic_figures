use std::cmp;

pub fn clip(x:u32,l:u32,h:u32) -> u32{
    if x<l{
        l
    } else if x>h{
        h
    } else{
        x as u32
    }
}

pub fn clip_add(x:u32,y:i32,l:u32,h:u32) -> u32{
    let s=x as i32 + y;
    if s<l as i32{
        return l;
    }
    return cmp::min(s as u32,h);
}

// pub fn hex_to_rgb(hex:&str) -> (u8,u8,u8){
//
// }
