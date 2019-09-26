use image::{RgbaImage,Rgba,Pixel};



pub fn average_color(target:&RgbaImage) -> (u8,u8,u8){
    let (mut sr,mut sg,mut sb,mut count)=(0,0,0,0);
    for tp in target.pixels(){
        if let &[r,g,b,_]=tp.channels(){
            sr+=r as usize;
            sg+=g as usize;
            sb+=b as usize;
            count+=1;
        }
    }
    ((sr/count) as u8,(sg/count) as u8,(sb/count) as u8)
}

pub fn compute_color(target:&RgbaImage,current:&RgbaImage,scanlines:&Vec<(u32,u32,u32)>,alpha:u8) -> Rgba<u8>{
    let (mut sr,mut sg,mut sb,mut count)=(0,0,0,0);
    let a=255f32/alpha as f32;
    for &(j,s,e) in scanlines{
        for i in s..=e{
            let tp=target[(i,j)].channels();
            let cp=current[(i,j)].channels();
            let tr=tp[0] as f32;
            let tg=tp[1] as f32;
            let tb=tp[2] as f32;
            let cr=cp[0] as f32;
            let cg=cp[1] as f32;
            let cb=cp[2] as f32;
            sr+=(a*(tr-cr)+cr) as u32;
            sg+=(a*(tg-cg)+cg) as u32;
            sb+=(a*(tb-cb)+cb) as u32;
            count+=1;
        }
    }
    Rgba([(sr/count) as u8,(sg/count) as u8,(sb/count) as u8,alpha])
}

pub fn full_cost(target:&RgbaImage,current:&RgbaImage) -> i32{
    let mut cost=0;
    for (tp,cp) in target.pixels().zip(current.pixels()){
        if let &[tr,tg,tb,ta]=tp.channels(){
            if let &[cr,cg,cb,ca]=cp.channels(){
                let dr=tr as i32-cr as i32;
                let dg=tg as i32-cg as i32;
                let db=tb as i32-cb as i32;
                let da=ta as i32-ca as i32;
                cost+=(dr*dr)+(dg*dg)+(db*db)+(da*da);
            }
        }
    }
    return cost
}

pub fn composite(target:&mut RgbaImage,scanlines:&Vec<(u32,u32,u32)>,color:&Rgba<u8>){
    // let Rgba{data:[sr,sg,sb,a]}=*color;
    // let sa=(a as f32)/255f32;
    for &(j,s,e) in scanlines{
        for i in s..=e{
            target[(i as u32,j as u32)].blend(color);
        }
    }
}

pub fn partial_cost(target:&RgbaImage,current:&RgbaImage,cost:i32,scanlines:&Vec<(u32,u32,u32)>,color:&Rgba<u8>)->i32{
    let mut new_cost=cost;
    for &(j,s,e) in scanlines{
        for i in s..=e{
            let tp=target[(i,j)];
            let cp=current[(i,j)];
            if let &[tr,tg,tb,ta]=tp.channels(){
                if let &[cr,cg,cb,ca]=cp.channels(){
                    let dr=tr as i32-cr as i32;
                    let dg=tg as i32-cg as i32;
                    let db=tb as i32-cb as i32;
                    let da=ta as i32-ca as i32;
                    new_cost-=(dr*dr)+(dg*dg)+(db*db)+(da*da);
                }
                //we blend a clone of current pixel with the color
                //then calculate its cost
                let mut cp=cp.clone();
                cp.blend(color);
                if let &[cr,cg,cb,ca]=cp.channels(){
                    let dr=tr as i32-cr as i32;
                    let dg=tg as i32-cg as i32;
                    let db=tb as i32-cb as i32;
                    let da=ta as i32-ca as i32;
                    new_cost+=(dr*dr)+(dg*dg)+(db*db)+(da*da);
                }
            }
        }
    }
    new_cost
}
