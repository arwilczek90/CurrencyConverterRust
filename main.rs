#[cfg(test)]
use mockito;




// Do the Mathmatical conversion
fn convert(amount: f64, rate: f64) -> f64{
    // Get a the real amount down to fraction of cents
    let full_amount = amount * rate;
    // Move the decimal point right 2 places
    let amount_shift_right = full_amount * 100.0;
    // Round up to the next cent
    let amount_round_to_cent = amount_shift_right.ceil();
    // Shift the decimal point to the left
    let amount_shift_left = amount_round_to_cent / 100.0;
    return amount_shift_left
}


fn main(){

}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_convert(){
        assert_eq!(convert(12.22,2.008), 24.54)
    }
}