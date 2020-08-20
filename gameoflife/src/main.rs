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
    let mut arr = vec![' '; length];
    let some: f32 = 0.429;
    let mut rng = rand::thread_rng();

    for i in 0..length{
        if(rng.gen_range(0.0, 1.0) > some)
        {
            arr[i] = 'c';
        }
        else
        {
            arr[i] = 'x';
        }
    }

    for i in 0..length{
        println!("{}", arr[i]);
        if((i as i16) !=0 && ((i as i16)+1) % rows == 0)
        {
            println!("\n");
        }
    }

    println!("array size: {}", arr.len());
}

fn live()
{
}


// random generator function
// 