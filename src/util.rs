use std::cmp::{min,max};
use super::scanline::Scanline;

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
    return min(s as u32,h);
}

//x1<=x2
fn bresenham_low(x1:i32,y1:i32,x2:i32,y2:i32) -> Vec<(usize,usize)>{
    let mut points:Vec<(usize,usize)>=Vec::new();
    let dx=x2-x1;
    let mut dy=y2-y1;
    let mut yinc=1;
    let mut y=y1;
    if dy<0{
        dy=-dy;
        yinc=-1;
    }
    let mut d=(-2*dy)+dx;
    for x in x1..=x2{
        points.push((x as usize,y as usize));
        if d<0{
            d+=2*dx;
            y+=yinc;
        }
        d-=2*dy;
    }
    points    
}

//y1<=y2
fn bresenham_high(x1:i32,y1:i32,x2:i32,y2:i32) -> Vec<(usize,usize)>{
    let mut points:Vec<(usize,usize)>=Vec::new();
    let dy=y2-y1;
    let mut dx=x2-x1;
    let mut xinc=1;
    let mut x=x1;
    if dx<0{
        dx=-dx;
        xinc=-1;
    }
    let mut d=(-2*dx)+dy;
    for y in y1..=y2{
        points.push((x as usize,y as usize));
        if d<0{
            d+=2*dy;
            x+=xinc;
        }
        d-=2*dx;
    }
    points
}

pub fn points_to_scanlines(points:&Vec<(usize,usize)>) -> Vec<Scanline>{
    let mut sl:Vec<Scanline>=Vec::new();
    let mut xstart=points[0].0;
    let mut prevx=points[0].0;
    let mut prevy=points[0].1;
    for &(x,y) in points {
        if y!=prevy{
            sl.push(Scanline::new(min(prevx,xstart),max(prevx,xstart),prevy));
            xstart=prevx;
        }
        prevy=y;
        prevx=x;
    }
    sl.push(Scanline::new(min(prevx,xstart),max(prevx,xstart),prevy));
    sl
}

pub fn bresenham(x1_u:u32,y1_u:u32,x2_u:u32,y2_u:u32) -> Vec<(usize,usize)>{ 
    let (x1,y1,x2,y2)=(x1_u as i32,y1_u as i32,x2_u as i32,y2_u as i32);
    if (y2-y1).abs()<(x2-x1).abs(){
        if x1>x2{
            bresenham_low(x2,y2,x1,y1)
        }
        else{
            bresenham_low(x1,y1,x2,y2)
        }
    }
    else{
        if y1>y2{
            bresenham_high(x2,y2,x1,y1)
        }
        else{
            bresenham_high(x1,y1,x2,y2)
        }
    }
}

pub fn quadratic_bezier(startx:u32,starty:u32,endx:u32,endy:u32,controlx:u32,controly:u32,t:f32) -> (u32,u32){
    let (x1,y1,x2,y2,x3,y3)=(startx as f32,starty as f32,endx as f32,endy as f32,controlx as f32,controly as f32);
    (bezier_aux(x1,x2,x3,t) as u32,bezier_aux(y1,y2,y3,t) as u32)
}

fn bezier_aux(pst:f32,pe:f32,con:f32,t:f32) -> f32{
    ((1.0-t)*(1.0-t)*pst)+(2.0*(1.0-t)*t*con)+(t*t*pe)
}

// pub fn hex_to_rgb(hex:&str) -> (u8,u8,u8){
//
// }
