struct Fish {
    timer: i32
}

fn main() {

    let mut fishes: Vec<Fish> = vec![];

    // Sample data
    let fish_timers = [ 3 , 4 , 3, 1, 2 ];
    let days = 80;

    // Real data
    let fish_timers = [ 5,1,5,3,2,2,3,1,1,4,2,4,1,2,1,4,1,1,5,3,5,1,5,3,1,2,4,4,1,1,3,1,1,3,1,1,5,1,5,4,5,4,5,1,3,2,4,3,5,3,5,4,3,1,4,3,1,1,1,4,5,1,1,1,2,1,2,1,1,4,1,4,1,1,3,3,2,2,4,2,1,1,5,3,1,3,1,1,4,3,3,3,1,5,2,3,1,3,1,5,2,2,1,2,1,1,1,3,4,1,1,1,5,4,1,1,1,4,4,2,1,5,4,3,1,2,5,1,1,1,1,2,1,5,5,1,1,1,1,3,1,4,1,3,1,5,1,1,1,5,5,1,4,5,4,5,4,3,3,1,3,1,1,5,5,5,5,1,2,5,4,1,1,1,2,2,1,3,1,1,2,4,2,2,2,1,1,2,2,1,5,2,1,1,2,1,3,1,3,2,2,4,3,1,2,4,5,2,1,4,5,4,2,1,1,1,5,4,1,1,4,1,4,3,1,2,5,2,4,1,1,5,1,5,4,1,1,4,1,1,5,5,1,5,4,2,5,2,5,4,1,1,4,1,2,4,1,2,2,2,1,1,1,5,5,1,2,5,1,3,4,1,1,1,1,5,3,4,1,1,2,1,1,3,5,5,2,3,5,1,1,1,5,4,3,4,2,2,1,3 ];
    //let days = 80;

    // create an array of our fish
    for fish_timer in fish_timers {
        let fish = Fish { timer: fish_timer };
        fishes.push(fish);
    }
    println!("{} TOTAL FISH", fishes.len());
    let mut day = 0;

    // Initial State
    print!("Initial State: ");
    for (_index, fish) in fishes.iter().enumerate() {
        print!("{},", fish.timer);
    }
    println!();

    for x in 0..days {

        // A day has passed, decrease the timer
        let mut add_fish_count = 0;
        for (_index, fish) in fishes.iter_mut().enumerate() {
            let mut timer = fish.timer;
            if timer != 0 {
                fish.timer = fish.timer -1;
            } else {
                // Reset fish's timer to six
                fish.timer = 6;
                add_fish_count += 1;
                // And create a new fish
                //let new_fish = Fish { timer: 8 };
                //fishes.push(new_fish);
            }
        }
        if add_fish_count > 0 {
            for _x in 0..add_fish_count {
                let new_fish = Fish { timer: 8 };
                fishes.push(new_fish);
            }
        }
        print!("After {} days: ", day+1);
        for (_index, fish) in fishes.iter().enumerate() {
             print!("{},",fish.timer);
        }
        println!();
        day += 1;
    }
    println!("{} TOTAL FISH", fishes.len());

}