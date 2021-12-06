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
    let fish_timers = [ 3 , 4 , 3, 1, 2 ];
    let days = 18;

    // Real data
    //let fish_timers = [ 5,1,5,3,2,2,3,1,1,4,2,4,1,2,1,4,1,1,5,3,5,1,5,3,1,2,4,4,1,1,3,1,1,3,1,1,5,1,5,4,5,4,5,1,3,2,4,3,5,3,5,4,3,1,4,3,1,1,1,4,5,1,1,1,2,1,2,1,1,4,1,4,1,1,3,3,2,2,4,2,1,1,5,3,1,3,1,1,4,3,3,3,1,5,2,3,1,3,1,5,2,2,1,2,1,1,1,3,4,1,1,1,5,4,1,1,1,4,4,2,1,5,4,3,1,2,5,1,1,1,1,2,1,5,5,1,1,1,1,3,1,4,1,3,1,5,1,1,1,5,5,1,4,5,4,5,4,3,3,1,3,1,1,5,5,5,5,1,2,5,4,1,1,1,2,2,1,3,1,1,2,4,2,2,2,1,1,2,2,1,5,2,1,1,2,1,3,1,3,2,2,4,3,1,2,4,5,2,1,4,5,4,2,1,1,1,5,4,1,1,4,1,4,3,1,2,5,2,4,1,1,5,1,5,4,1,1,4,1,1,5,5,1,5,4,2,5,2,5,4,1,1,4,1,2,4,1,2,2,2,1,1,1,5,5,1,2,5,1,3,4,1,1,1,1,5,3,4,1,1,2,1,1,3,5,5,2,3,5,1,1,1,5,4,3,4,2,2,1,3 ];
    //let days = 256;

    // see how many unique timers we have


    // Create another array to hold the total unique fish timer counts in the fish timers
    let mut unique_fish_counts = [0_u64; 9];
    for fish_timer in fish_timers {
        unique_fish_counts[fish_timer] += 1;
    }

    // before
    println!("BEFORE: {:?}", unique_fish_counts);

    for _day in 0..days {
        unique_fish_counts.rotate_left(1);
        unique_fish_counts[6] += unique_fish_counts[8];
        println!("  DAY {}: {:?}", _day+1, unique_fish_counts);
    }

    // after
    println!("AFTER: {:?}", unique_fish_counts); // oh my...

    // With the sample scenario and data: 3,4,3,1,2
    // from part one, looks like this before and after 18 and 80 days

    // 18 days
    // BEFORE: [0, 1, 1, 2, 1, 0, 0, 0, 0]
    // AFTER: [3, 5, 3, 2, 2, 1, 5, 1, 4]
    // Total fish count after 18 days is 26
    // Look at the last line, day 18 in the puzzle, and compare the type counts to the after
    // array and you will see they line up

    // 80 days
    // BEFORE: [0, 1, 1, 2, 1, 0, 0, 0, 0]
    // AFTER: [424, 729, 558, 790, 739, 762, 991, 370, 571]
    // Total fish count after 80 days is 5934

    // sum up all the fish in the resulting array and print it out
    let total_fish: u64 = unique_fish_counts.into_iter().sum::<u64>();
    println!("Total fish count after {} days is {}", days, total_fish);
}

fn main() {
    part_one();
    part_two();
    array_visualization();
}

fn array_visualization() {

    println!("\n Some visualization of what the rotate add function is doing");

    let mut day_17_array  =  [4, 3, 5, 3, 2, 2, 1, 1, 1]; // DAY 17 with sample data
    println!("DAY 17: {:?}", day_17_array );

    // now we rotate it left
    day_17_array.rotate_left(1);
    println!("DAY 17: {:?}  (after .rotate_left(1))", day_17_array );

    // now we add
    day_17_array[6] += day_17_array[8];
    let mut day_18_array = day_17_array;
    println!("DAY 18: {:?}  (after field 8 added to field 6)", day_18_array );

    // now sum up the fields
    println!("TOTALS: {}", day_18_array.into_iter().sum::<u64>());
}
