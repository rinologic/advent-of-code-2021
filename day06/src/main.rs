struct Fish {
    timer: i32
}


fn part_one() {

    println!("\nPART ONE");

    let mut fishes: Vec<Fish> = vec![];

    // Sample data
    //let fish_timers = [ 3 , 4 , 3, 1, 2 ];
    //let days = 80;

    // Real data
    let fish_timers = [ 5,1,5,3,2,2,3,1,1,4,2,4,1,2,1,4,1,1,5,3,5,1,5,3,1,2,4,4,1,1,3,1,1,3,1,1,5,1,5,4,5,4,5,1,3,2,4,3,5,3,5,4,3,1,4,3,1,1,1,4,5,1,1,1,2,1,2,1,1,4,1,4,1,1,3,3,2,2,4,2,1,1,5,3,1,3,1,1,4,3,3,3,1,5,2,3,1,3,1,5,2,2,1,2,1,1,1,3,4,1,1,1,5,4,1,1,1,4,4,2,1,5,4,3,1,2,5,1,1,1,1,2,1,5,5,1,1,1,1,3,1,4,1,3,1,5,1,1,1,5,5,1,4,5,4,5,4,3,3,1,3,1,1,5,5,5,5,1,2,5,4,1,1,1,2,2,1,3,1,1,2,4,2,2,2,1,1,2,2,1,5,2,1,1,2,1,3,1,3,2,2,4,3,1,2,4,5,2,1,4,5,4,2,1,1,1,5,4,1,1,4,1,4,3,1,2,5,2,4,1,1,5,1,5,4,1,1,4,1,1,5,5,1,5,4,2,5,2,5,4,1,1,4,1,2,4,1,2,2,2,1,1,1,5,5,1,2,5,1,3,4,1,1,1,1,5,3,4,1,1,2,1,1,3,5,5,2,3,5,1,1,1,5,4,3,4,2,2,1,3 ];
    let days = 80;

    // create an array of our fish
    for fish_timer in fish_timers {
        let fish = Fish { timer: fish_timer };
        fishes.push(fish);
    }
    println!("{} TOTAL FISH", fishes.len());

    // Initial State
    print!("Initial State: ");
    for (_index, fish) in fishes.iter().enumerate() {
        print!("{},", fish.timer);
    }
    println!();

    let mut day = 0;
    for x in 0..days {
        // A day has passed, decrease the timer
        let mut add_fish_count = 0;
        for (_index, fish) in fishes.iter_mut().enumerate() {
            let mut timer = fish.timer;
            if timer != 0 {
                fish.timer = fish.timer -1;
            } else {
                // reset fish's timer to six
                fish.timer = 6;
                add_fish_count += 1; // have to push outside of loop
            }
        }
        if add_fish_count > 0 {
            for _x in 0..add_fish_count {
                let new_fish = Fish { timer: 8 };
                fishes.push(new_fish);
            }
        }
        // Only uncomment this if the number of days are small!
        //print!("After {} days: ", day+1);
        // for (_index, fish) in fishes.iter().enumerate() {
        //      print!("{},",fish.timer);
        // }
        //println!();
        day += 1;
        println!("DAY {} COMPLETE", day);
    }
    println!("{} TOTAL FISH", fishes.len());
}

fn part_two() {

    println!("\nPART TWO");

    // Sample data
    //let fish_timers = [ 3 , 4 , 3, 1, 2 ];
    //let days = 80;

    // Real data
    let fish_timers = [ 5,1,5,3,2,2,3,1,1,4,2,4,1,2,1,4,1,1,5,3,5,1,5,3,1,2,4,4,1,1,3,1,1,3,1,1,5,1,5,4,5,4,5,1,3,2,4,3,5,3,5,4,3,1,4,3,1,1,1,4,5,1,1,1,2,1,2,1,1,4,1,4,1,1,3,3,2,2,4,2,1,1,5,3,1,3,1,1,4,3,3,3,1,5,2,3,1,3,1,5,2,2,1,2,1,1,1,3,4,1,1,1,5,4,1,1,1,4,4,2,1,5,4,3,1,2,5,1,1,1,1,2,1,5,5,1,1,1,1,3,1,4,1,3,1,5,1,1,1,5,5,1,4,5,4,5,4,3,3,1,3,1,1,5,5,5,5,1,2,5,4,1,1,1,2,2,1,3,1,1,2,4,2,2,2,1,1,2,2,1,5,2,1,1,2,1,3,1,3,2,2,4,3,1,2,4,5,2,1,4,5,4,2,1,1,1,5,4,1,1,4,1,4,3,1,2,5,2,4,1,1,5,1,5,4,1,1,4,1,1,5,5,1,5,4,2,5,2,5,4,1,1,4,1,2,4,1,2,2,2,1,1,1,5,5,1,2,5,1,3,4,1,1,1,1,5,3,4,1,1,2,1,1,3,5,5,2,3,5,1,1,1,5,4,3,4,2,2,1,3 ];
    let days = 256;

    // Create another array to hold the total unique fish timer counts in the fish timers
    let mut unique_fish_counts = [0_u64; 9];
    for fish_timer in fish_timers {
        unique_fish_counts[fish_timer] += 1;
    }

    //println!("{:?}", total_fish);

    // for _day in 0..days {
    //     unique_fish_counts.rotate_left(1);
    //     println!("{:?}", unique_fish_counts);
    // }

    for _day in 0..days {
        unique_fish_counts.rotate_left(1);
        unique_fish_counts[6] += unique_fish_counts[8];
    }

    let total_fish: u64 = unique_fish_counts.into_iter().sum::<u64>();
    println!("Fish count after {} days is {}", days, total_fish);
}

fn main() {
    part_one();
    part_two();
}
