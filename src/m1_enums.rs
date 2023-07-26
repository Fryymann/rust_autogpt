
#[derive(Debug)]
enum CarColor {
    Red,
    Green,
    Blue,
    Silver
}

#[derive(Debug)]
enum GivenResult<T, E> {
    Ok(T),
    Err(E)
}

fn create_car_color_blue() -> CarColor {
    let my_car_color: CarColor = CarColor::Blue;
    my_car_color
}

fn check_under_five(num_check: u8) -> GivenResult<u8, String> {
    if num_check < 5 {
        GivenResult::Ok(num_check)
    } else {
        GivenResult::Err("Not under 5!".to_string())
    }
}




#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_enums() {
        let car_color:CarColor = create_car_color_blue();
        dbg!(car_color);

        let under_five_res: GivenResult<u8, String> = check_under_five(2);
        dbg!(under_five_res);

        let under_five_res: GivenResult<u8, String> = check_under_five(7);
        dbg!(under_five_res);
    }
}