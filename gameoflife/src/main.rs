extern crate rand;
use rand::Rng;

fn main() {
    let rows:i16 = 5;
    let cols:i16 = 4;
    let generations:i16 = 4;
    life(rows, cols, generations)
}

fn life(rows:i16, cols:i16, generations:i16){
    let length :usize = ((rows * cols)) as usize;
    let mut arr = vec![0; length];
    let some: f32 = 0.429;
    let mut rng = rand::thread_rng();

    for i in 0..length{
        if rng.gen_range(0.0, 1.0) > some
        {
            arr[i] = 1;
        }
        else
        {
            arr[i] = 0;
        }
    }

    live(arr.as_mut_slice(), rows, cols, generations);
}



fn live(a:&mut [i16], r:i16, cols:i16, mut gen:i16) { 
    if gen<1 {
        return;
    }

    println!("Generation {}\n", gen);
    for i in 0..a.len(){
        print!("{}", a[i as usize]);
        if (i as i16) !=0 && ((i as i16)+1) % r == 0
        {
            println!();
        }
    }

    let mut b = vec![0; a.len()];
    for x in 0..a.len() {
        let mut c_left: i16 = 0;
        let mut c_right: i16 = 0; 
        
        let mut c_top: i16 = 0;
        let mut c_top_left: i16 = 0;
        let mut c_top_right: i16 =0;

        let mut c_bottom: i16 = 0;
        let mut c_bottom_left: i16 = 0;
        let mut c_bottom_right: i16 =0;
        
        if x as i16 - 1 >= 0 {
            c_left = a[x-1];
        }

        if x as i16 +1 < r {
            c_right = a[x+1]; 
        }

        if x as i16 - r  >= 0{
            c_top = a[x - (r as usize)];
        }

        if x % (r as usize) != 0 && x as i16 - r  -1 > 0{
            c_top_left = a[x - (r as usize) -1];
        } 

        if (x + 1) % (r as usize) != 0 && x as i16 - r + 1 >= 0  {
            c_top_right = a[x - (r as usize) + 1];
        } 

        if x + (r as usize) < a.len() {
            c_bottom = a[x + (r as usize)];
        }

        if x % (r as usize) != 0 && x + (r as usize) -1 < a.len(){
            c_bottom_left = a[x + (r as usize) -1];
        } 

        if x + 1 % (r as usize) != 0 && x + (r as usize) + 1 < a.len()  {
            c_bottom_right = a[x + (r as usize) + 1];
        } 

        
        let _neighbors: i16 = c_left + c_right + c_top + c_top_left + c_top_right + c_bottom + c_bottom_left + c_bottom_right;
        b[x] = a[x];
        if a[x] == 1
        {
            if _neighbors < 2{
                b[x] = 0;
            }
            else if _neighbors > 3{
                b[x] = 0;
            }
             

        }
        else
        {
            if _neighbors == 3{
                b[x] = 1;
            }
        }
        
    }
    
    gen -= 1;
    live(b.as_mut_slice(), r, cols, gen);
    
    
    
}

