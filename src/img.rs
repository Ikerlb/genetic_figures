use super::SIZE;
use image::{Pixel, Rgba, RgbaImage};
use super::scanline::Scanline;

use std::fmt;

#[derive(Debug,Clone)]
pub struct Color{
    pub r:u8,
    pub g:u8,
    pub b:u8,
    pub a:u8,
}

impl Color{
    pub fn new(r:u8,g:u8,b:u8,a:u8) -> Color{
        Color{
            r,g,b,a
        }
    }

    pub fn from_slice(s:&[u8]) -> Color{
        let r=s[0];
        let g=s[1];
        let b=s[2];
        let a=s[3];
        Color{
            r,g,b,a
        }
    }

    pub fn normalize(&self) -> (f64,f64,f64,f64){
        let (r,g,b,a)=self.unpack_as_f64();
        (r/255.0,g/255.0,b/255.0,a/255.0)
    }

    pub fn unpack(&self) -> (u8,u8,u8,u8){
        (self.r,self.g,self.b,self.a)    
    }

    pub fn unpack_as_f64(&self) -> (f64,f64,f64,f64){
        (self.r as f64,self.g as f64,self.b as f64,self.a as f64)
    }

    pub fn unpack_as_f32(&self) -> (f32,f32,f32,f32){
        (self.r as f32,self.g as f32,self.b as f32,self.a as f32)
    }

    pub fn unpack_as_i32(&self) -> (i32,i32,i32,i32){
        (self.r as i32,self.g as i32,self.b as i32,self.a as i32)
    }

    pub fn blend(&mut self,bg:&Color){
        //http://stackoverflow.com/questions/7438263/alpha-compositing-algorithm-blend-modes#answer-11163848
        //alpha_final = alpha_bg + alpha_fg - alpha_bg * alpha_fg
        let (r_bg,g_bg,b_bg,a1)=self.unpack_as_f32();
        let (r_fg,g_fg,b_fg,a2)=bg.unpack_as_f32();
        let (a_bg,a_fg)=(a1/255.0,a2/255.0);
        //get final alpha
        let alpha_final=a_bg+a_fg-a_bg*a_fg;
        //premultiply by alpha
        let (r_bg_a,g_bg_a,b_bg_a)=(r_bg*a_bg, g_bg*a_bg, b_bg*a_bg);
        let (r_fg_a,g_fg_a,b_fg_a)=(r_fg*a_fg, g_fg*a_fg, b_fg*a_fg);

        //calculate final color
        let (r_fin_a,g_fin_a,b_fin_a)=(
            r_fg_a+r_bg_a*(1.0-a_fg),
            g_fg_a+g_bg_a*(1.0-a_fg),
            b_fg_a+b_bg_a*(1.0-a_fg)
        );
        
        //unmultiply alpha
        self.r=(r_fin_a/alpha_final) as u8;
        self.g=(g_fin_a/alpha_final) as u8;
        self.b=(b_fin_a/alpha_final) as u8;
        self.a=(alpha_final*255.0) as u8;
    }


}

pub struct Img{
    pub buf: [u8;Img::BSIZE],
    pub w: usize,
    pub h: usize,
}

impl fmt::Display for Img{
    fn fmt(&self,f: &mut fmt::Formatter)->fmt::Result{
        for i in 0..self.w{
            for j in 0..self.h{
                writeln!(f,"{:?}",self.get(i,j))?
            }
        }
        Ok(())
    } 
}

impl std::cmp::PartialEq for Img{
    fn eq(&self, other: &Self) -> bool {
        if self.w != other.w || self.h != other.h{
            return false;
        }
        for (s,o) in self.buf.iter().zip(other.buf.iter()){
            if s != o{
                return false;
            }
        }
        true
    }
}

impl std::cmp::Eq for Img {}

impl Img{
    //w*h*4 as we are using rgba channels
    pub const BSIZE:usize=SIZE*SIZE*4;

    fn index(&self,x:usize,y:usize) -> usize{
        4*((y*self.w)+x)
    }

    //self is current
    pub fn compute_color(&self,target:&Img,scanlines:&[Scanline],alpha:u8) -> Color{
        let (mut sr,mut sg,mut sb,mut count)=(0,0,0,0);
        let a=255f32/alpha as f32;
        for sl in scanlines{
            let (j,s,e) = sl.unpack();
            for i in s..=e{
                let tp=target.get_color(i,j);
                let cp=self.get_color(i,j);

                let (cr,cg,cb,ca)=cp.unpack_as_f32();
                let (tr,tg,tb,ta)=tp.unpack_as_f32();
            
                sr+=(a*(tr-cr)+cr) as u32;
                sg+=(a*(tg-cg)+cg) as u32;
                sb+=(a*(tb-cb)+cb) as u32;
                count+=1;
            }
        }
        Color::new((sr/count) as u8,(sg/count) as u8,(sb/count) as u8,alpha)
    }

    pub fn composite(&mut self,scanlines:&[Scanline],color:&Color){
        for sl in scanlines{
            let (j,s,e) = sl.unpack();
            for i in s..=e{
                let mut c=self.get_color(i,j);
                //println!("c: {:?} color {:?}",c,color);
                c.blend(&color);
                self.set_color(i,j,&c);
            }
        }
    }

    //self is current
    pub fn partial_cost(&self,target:&Img,cost:i32,scanlines:&[Scanline],color:&Color) -> i32{
        let mut new_cost=cost;
        for sl in scanlines{
            let (j,s,e) = sl.unpack();
            for i in s..=e{
                let tp=target.get_color(i,j);
                let cp=self.get_color(i,j);

                let (tr,tg,tb,ta)=tp.unpack_as_i32();
                let (cr,cg,cb,ca)=cp.unpack_as_i32();

                let dr=tr-cr;
                let dg=tg-cg;
                let db=tb-cb;
                let da=ta-ca;

                new_cost-=(dr*dr)+(dg*dg)+(db*db)+(da*da);

                //we blend a clone of current pixel with the color.
                // then we proceed to calculate its cost
                let mut cp=cp.clone();
                cp.blend(color);
                let (cr_b,cg_b,cb_b,ca_b)=cp.unpack_as_i32();
                
                let dr_b=tr-cr_b;
                let dg_b=tg-cg_b;
                let db_b=tb-cb_b;
                let da_b=ta-ca_b;

                new_cost+=(dr_b*dr_b)+(dg_b*dg_b)+(db_b*db_b)+(da_b*da_b);
            }
        }
        new_cost
    }

    pub fn average_color(&self,alpha:u8) -> Color{
        let (mut sr,mut sg,mut sb,mut count)=(0,0,0,0);
        for j in 0..self.h{
            for i in 0..self.w{
                let c=self.get_color(i,j);
                sr+=c.r as usize;
                sg+=c.g as usize;
                sb+=c.b as usize;
                count+=1;
            }
        }
        Color::new((sr/count) as u8,(sg/count) as u8,(sb/count) as u8,alpha)
    }

    pub fn full_cost(&self,target:&Img) -> i32{
        let mut cost=0;
        for j in 0..self.h{
            for i in 0..self.w{
                let t=target.get_color(i,j);
                let c=self.get_color(i,j);

                let dr=t.r as i32-c.r as i32;
                let dg=t.g as i32-c.g as i32;
                let db=t.b as i32-c.b as i32;
                let da=t.a as i32-c.a as i32;

                cost+=(dr*dr)+(dg*dg)+(db*db)+(da*da);
            }
        }
        return cost;
    }


    pub fn get_color(&self,x:usize,y:usize)->Color{
        let n=self.index(x,y);
        Color::from_slice(&self.buf[n..n+4])
    }

    pub fn get(&self,x:usize,y:usize) -> u8{
        let n=self.index(x,y);
        self.buf[n]
    }

    pub fn set_color(&mut self,x:usize,y:usize,color:&Color) {
        let n=self.index(x,y);
        self.buf[n]=color.r;
        self.buf[n+1]=color.g;
        self.buf[n+2]=color.b;
        self.buf[n+3]=color.a;
    }

    pub fn new()->Img{
        Img{
            buf:[0; Img::BSIZE],
            w: SIZE,
            h: SIZE,
        }
    }

    pub fn from_rgba_image(target:&RgbaImage)->Img{
        let mut idx=0;
        let (tw,th)=target.dimensions();
        assert!((tw*th*4) as usize==Img::BSIZE);
        let mut buf=[0;Img::BSIZE];
        for j in 0..SIZE{
            for i in 0..SIZE{
                let tp=target.get_pixel(i as u32,j as u32);
                if let &[r,g,b,a]=tp.channels(){
                    buf[idx]=r;
                    buf[idx+1]=g;
                    buf[idx+2]=b;
                    buf[idx+3]=a;
                }
                idx+=4;
            }
        }
        Img{
            buf,
            w: SIZE,
            h: SIZE,
        }
    }

    pub fn from_fn<F>(f:F)->Img
    where
        F:Fn(usize,usize)->(u8,u8,u8,u8) 
    {
        let mut buf=[0;Img::BSIZE];
        let mut idx=0;
        for j in 0..SIZE{
            for i in 0..SIZE{
                let (r,g,b,a)=f(i,j);
                buf[idx]=r;
                buf[idx+1]=g;
                buf[idx+2]=b;
                buf[idx+3]=a;
                idx+=4
            }
        }
        Img{
            buf,
            w: SIZE,
            h: SIZE,
        }      
    }
    
    pub fn print(&self){
        for i in &self.buf[..]{
            print!("{},",i);
        }
    }

}
