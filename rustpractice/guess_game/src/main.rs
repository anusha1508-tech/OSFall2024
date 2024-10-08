// Function to check if the guess is correct, too high, or too low
fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0 // Correct guess
    } else if guess > secret {
        1 // Guess is too high
    } else {
        -1 // Guess is too low
    }
}

fn main() {
    // Hard-coded secret number (the number to guess)
    let secret_number = 42;

    // Variable to track the number of guesses
    let mut num_guesses = 0;
    
    // Starting guess
    let mut guess = 38;

    // Loop to repeatedly guess
    loop {
        num_guesses += 1;

        // Check the guess using the check_guess function
        let result = check_guess(guess, secret_number);

        // Use if-else expression to print the result and break if correct
        if result == 0 {
            println!("Your guess of {} is correct!", guess);
            break;
        } else if result == 1 {
            println!("Your guess of {} is too high.", guess);
        } else {
            println!("Your guess of {} is too low.", guess);
        }

        // Increment the guess value for the next iteration
        guess += 1;
    }

    // Print how many guesses it took
    println!("It took you {} guesses to guess the correct number.", num_guesses);
}
