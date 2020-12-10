use rand::{thread_rng,Rng};

fn main() {

    let reps: i32 = 10000;

    let mut win: i32 = 0;
    let mut loss: i32 = 0;

    let mut rng = thread_rng();
    
   for _i in 0..reps {

       let door1 = rng.gen_range(0,3);
       let mut door2 = rng.gen_range(0,3);
        while door2 == door1 {
            door2 = rng.gen_range(0,3);
        }
       let mut door3 = rng.gen_range(0,3);
        while door2 == door3 || door1 == door3 {
            door3 = rng.gen_range(0,3);
        }

    let choice = rng.gen_range(1,4);

    if choice == 1  && door2 != 0 {
        if door3 == 0 {
            win += 1;
        }else{
            loss +=1;
        }
    }else if choice == 1 && door3 != 0 {
        if door2 == 0 {
            win += 1;
        }else{
            loss += 1;
        }

    }

    if choice == 2  && door1 != 0 {
        if door3 == 0 {
            win += 1;
        }else{
            loss +=1;
        }
    }else if choice == 2 && door3 != 0 {
        if door1 == 0 {
            win += 1;
        }else{
            loss += 1;
        }

    }

    if choice == 3  && door2 != 0 {
        if door1 == 0 {
            win += 1;
        }else{
            loss +=1;
        }
    }else if choice == 3 && door1 != 0 {
        if door2 == 0 {
            win += 1;
        }else{
            loss += 1;
        }

    }




   }


println!("Wins: {}",win);
println!("Loss: {}",loss);
println!("Win/Repeitions: {}", (win as f64)/(reps as f64));
println!("Win/Loss: {}", (win as f64)/(loss as f64));

}
