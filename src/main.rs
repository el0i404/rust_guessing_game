use std::io;
use rand::Rng;

fn main() {
    println!("🎯 Enhanced Guessing Game 🎯");

    // Select difficulty
    let difficulty = select_difficulty();
    let (min, max, max_attempts) = get_difficulty_settings(difficulty);

    println!("I'm thinking of a number between {} and {}!", min, max);
    println!("You have {} attempts to guess it!", max_attempts);

    // Generate secret number
    let secret_number = rand::rng().random_range(min..=max);

    // Game loop
    let mut attempts = 0;
    let mut won = false;

    while attempts < max_attempts && !won {
        attempts += 1;
        println!("\nAttempt {}/{}", attempts, max_attempts);
        print!("Enter your guess: ");

        let guess = get_user_guess();

        match guess {
            Some(num) if num == secret_number => {
                println!("🎉 Congratulations! You guessed it in {} attempts!", attempts);
                won = true;
            },
            Some(num) => {
                give_feedback(num, secret_number, attempts, max_attempts);
            },
            None => {
                println!("❌ Invalid input! Please enter a number.");
                attempts -= 1; // Don't count invalid attempts
            }
        }
    }

    if !won {
        println!("💀 Game over! The number was {}.", secret_number);
    }

    calculate_score(won, attempts, max_attempts, difficulty);
}

fn select_difficulty() -> u32 {
    // TODO: Display difficulty menu and get user choice
    let mut entry: String = String::new();
    println!("Press 1 for Easy \nPress 2 for Medium \nPress 3 for Hard \nPress 4 for Expert");
    io::stdin().read_line(&mut entry).expect("Wrong entry");

    match entry.trim().parse::<u32>() {
        Ok(n) if (1..=4).contains(&n) => return n,
        _ => println!("Invalid choice, please enter a number between 1 and 4."),
    }

    entry.to_string().parse().expect("wrong")
    // 1. Easy (1-50, 10 attempts)
    // 2. Medium (1-100, 8 attempts)
    // 3. Hard (1-200, 6 attempts)
    // 4. Expert (1-500, 5 attempts)
    // 1
}

fn get_difficulty_settings(difficulty: u32) -> (i32, i32, u32) {
    // TODO: Return (min, max, max_attempts) based on difficulty
    let (min, max, max_attempts) = match difficulty {
        1 => (1, 50, 10),   // Easy
        2 => (1, 100, 8),   // Medium
        3 => (1, 200, 6),   // Hard
        4 => (1, 500, 5),   // Expert
        _ => (1, 50, 10),   // Default to easy
    };

    (min, max, max_attempts)
}

fn get_user_guess() -> Option<i32> {
    // TODO: Get and parse user input, return None for invalid input
    let mut entry: String = String::new();

    io::stdin().read_line(&mut entry).expect("Wrong entry");

    match entry.trim().parse::<i32>() {
        Ok(n) if (1..=4).contains(&n) => return Some(n),
        _ => None,
    }

}

fn give_feedback(guess: i32, secret: i32, attempts: u32, max_attempts: u32) {
    // TODO: Provide helpful feedback:
    // - Too high/low
    if guess < secret {
        println!("{} is too low", guess);
    }
    if guess > secret {
        println!("{} is too high", guess);
    }
    // - How close they are
    let diff = guess.wrapping_sub(secret);
    println!("You are {} numbers close", diff);

    // - Remaining attempts
    let remaining_attempts = max_attempts - attempts;
    println!("You have {} remaining attemps", remaining_attempts);
    // - Hints based on attempts remaining
}

fn calculate_score(won: bool, attempts: u32, max_attempts: u32, difficulty: u32) {
    // TODO: Calculate and display score based on:
    // - Whether they won
    
    if won { println!("You won! :)"); }
    // - Number of attempts used
    let diff = max_attempts as i32 - attempts as i32;

    println!("You have {} remaining attempts", diff);
    // - Difficulty level
    println!("Your Difficulty {} ", difficulty);

}
