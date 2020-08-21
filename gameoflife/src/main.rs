extern crate rand;
use rand::Rng;

fn main() {
    let rows:i16 = 5;
    let cols:i16 = 4;
    let mut generations:i16 = 4;
    life(rows, cols, generations)
}

fn life(rows:i16, cols:i16, mut generations:i16){
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
        if(x-1 < 0){
            let c_m1:i16 = 0;
        }
        else{
            let c_m1:i16 = a[x-1];
        }

        let c_p1: usize = x+1;
        let c_mr_m1: usize = x-r as usize -1;
        let c_mr: usize = x-r as usize;
        let c_mr_p1: usize = x-r as usize +1;
        let c_pr_m1: usize = x+r as usize -1;
        let c_pr: usize = x+r as usize;
        let c_pr_p1: usize = x+r as usize +1;
        let neighbors: i16 = a[c_p1 as usize]+a[c_mr_m1 as usize]+a[c_mr as usize]+a[c_mr_p1 as usize]+a[c_pr_m1 as usize]+a[c_pr as usize]+a[c_pr_p1 as usize];
        
        b[x] = a[x];
    }
    if(gen > 1)
    {
        gen -= 1;
        live(b.as_mut_slice(), r, cols, gen);
    }
    
    
}

