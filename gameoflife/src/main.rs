extern crate rand;
use rand::Rng;
use std::io::{self, Write};
use std::{thread, time};

fn main() {
    clear();
    let rows:i16 = 50;
    let cols:i16 = 20;
    let generations:i16 = 10;
    life(rows, cols, generations)
}

fn life(rows:i16, cols:i16, generations:i16){
    let length :usize = ((rows * cols)) as usize;
    let mut arr = vec![0; length];
    let some: f32 = 0.629;
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

    clear();
    println!();
    println!("Generation {}\n", gen);
    for i in 0..a.len(){
        if a[i as usize] == 1 {
            print!("o")
        }
        else{
            print!(" ")
        }

        if (i as i16) !=0 && ((i as i16)+1) % cols == 0
        {
            println!();
        }
    }
    thread::sleep(time::Duration::from_millis(200));

    let mut b = vec![0; a.len()];
    print!("{}", a.len());
    for x in 0..a.len() {
        let mut c_left: i16 = 0;
        let mut c_right: i16 = 0; 
        
        let mut c_top: i16 = 0;
        let mut c_top_left: i16 = 0;
        let mut c_top_right: i16 =0;

        let mut c_bottom: i16 = 0;
        let mut c_bottom_left: i16 = 0;
        let mut c_bottom_right: i16 =0;
        
        if x % (cols as usize) != 0 {
            c_left = a[x-1];
        }

        if (x+1) % (cols as usize) !=0 {
            c_right = a[x+1]; 
        }

        if x as i16 - r  >= 0{
            c_top = a[x - (r as usize)];
        }

        if x % (cols as usize) != 0 && x as i16 - cols  -1 > 0{
            c_top_left = a[x - (cols as usize) -1];
        } 

        if x as i16 - cols + 1 >= 0 && (x + 1) % (cols as usize) != 0 {
            c_top_right = a[x - (cols as usize) + 1];
        } 

        if x + (cols as usize) < a.len() {
            c_bottom = a[x + (cols as usize)];
        }

        if x % (cols as usize) != 0 && x + (cols as usize) -1 < a.len(){
            c_bottom_left = a[x + (cols as usize) -1];
        } 

        if (x + 1) % (cols as usize) != 0 && x + (cols as usize) + 1 < a.len()  {
            c_bottom_right = a[x + (cols as usize) + 1];
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

pub fn clear() {
    io::stdout().write_all("\x1b[2J\x1b[1;1H".as_bytes()).unwrap()
}

