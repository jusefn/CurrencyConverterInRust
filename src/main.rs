mod convert;

const CURRENCY_CHARACTERS: [char; 2] =  ['€','$'];


// Main function
fn main() {

    let input_currency: usize = 0;
    let output_currency: usize = 1;
    let amount: f32 = 30.0;
    let output = convert::exchange(&input_currency, &output_currency, &amount);

    println!("{:.2}{}", output, CURRENCY_CHARACTERS[output_currency]);
    

}