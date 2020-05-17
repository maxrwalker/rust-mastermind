use std::io;

fn main(){
    println!("Enter the secret 4 digit code and number of guesses allowed in format: [xxxx x]");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the secret code");
    let secret = &input[0..4];
    let guesses_allowed = &input[5..(input.len()-2)].parse::<i32>().unwrap(); //<- seems to need -2 to remove he line break and carriage return?
    println!("The secret is {}", secret);
    println!("Guesses: [{}]", guesses_allowed);

    for _x in 0..*guesses_allowed{
       let mut guess = "".to_string(); //<--another way equivilent to: String::new()
       println!("Enter your guess in the format: [xxxx]");
       io::stdin().read_line(&mut guess).expect("Failed to read the guess from user input");
       let guess_result = check_guess(&guess[..], secret);
       println!("{}-{}", guess_result.0, guess_result.1);
       if guess_result.0 == 4 {
           println!("You did it you genius!");
           break;
       }
    }
}

//Function checks a single guess
//Takes a secret and a guess and returns the result tuple (correct, close/wrong position)
fn check_guess(guess : &str, secret : &str) -> (i32, i32) {
    let mut correct_cnt = 0;
    let mut close_cnt = 0;
    let mut secret_char_idx_used = [false, false, false, false]; //marks digits of the secret as used.
    //iterate the guess digits and find the correct matches:
    for (i,_j) in (0..4).enumerate() {
        let guess_char = &guess[i..i+1];
        if &secret[i..i+1] == guess_char {
            secret_char_idx_used[i] = true;
            correct_cnt += 1;            
        }
    }
    //iterate the secret digits and find the 'right digit in the wrong position' count:
    for (i,_j) in (0..4).enumerate() {
        let secret_char = &secret[i..i+1];
        if secret_char_idx_used[i] {
            continue; //this secret digits already guessed correctly.
        }
        for(ii,_jj) in (0..4).enumerate(){
            let guess_char = &guess[ii..ii+1];
            if secret_char == guess_char{
                secret_char_idx_used[i] = true;
                close_cnt += 1;
                break;
            }
        }
    }
    let result = (correct_cnt, close_cnt);
    result
}
