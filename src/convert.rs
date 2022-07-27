pub fn exchange(input_currency: &usize, output_currency: &usize, amount: &f32) -> f32 {


    if input_currency == &0 && output_currency == &1 {
        return convert_euro_to_dollar(&amount);
    } else if input_currency == &1 && output_currency == &0 {
        return convert_dollar_to_euro(&amount);
    } 

    return 0.0;

}

fn convert_euro_to_dollar(amount: &f32) -> f32{
    amount * 1.01450
}

fn convert_dollar_to_euro(amount: &f32) -> f32{
    amount * 0.98570
}