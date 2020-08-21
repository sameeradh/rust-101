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
        if(rng.gen_range(0.0, 1.0) > some)
        {
            arr[i] = 1;
        }
        else
        {
            arr[i] = 0;
        }
    }

    for i in 0..length{
        print!("{}", arr[i]);
        if((i as i16) !=0 && ((i as i16)+1) % rows == 0)
        {
            println!();
        }
    }
    live(arr.as_mut_slice(), rows, cols, generations);
}



fn live(a:&mut [char], r:i16, cols:i16, gen:i16) { 
    if gen<1 {
        return false;
    }
    println!("Generation {}\n", gen);
    for i in 0..a.len(){
        print!("{}", a[i as usize]);
        if((i as i16) !=0 && ((i as i16)+1) % r == 0)
        {
            println!();
        }
    }

   // for c in 0..a.len() {
    //    match a[c] {
      //      0 => println!(" ");
        //    1 => println!("o");
       // }
       // if !(c%r) { println!("\n");}
   // }
    
}

fn logic(a: &[i16], c: usize, r: usize) -> &[i16] {
    let mut b = vec![0; a.len()];
    for c in 0..a.len() {
        let c_m1: usize = c-1;
        let c_p1: usize = c+1;
        let c_mr_m1: usize = c-r-1;
        let c_mr: usize = c-r;
        let c_mr_p1: usize = c-r+1;
        let c_pr_m1: usize = c+r-1;
        let c_pr: usize = c+r;
        let c_pr_p1: usize = c+r+1;
        let neighbors: i16 = a[c_m1 as usize]+a[c_p1 as usize]+a[c_mr_m1 as usize]+a[c_mr as usize]+a[c_mr_p1 as usize]+a[c_pr_m1 as usize]+a[c_pr as usize]+a[c_pr_p1 as usize];
        
        b[c] = a[c];

    }




}


// random generator function
// 