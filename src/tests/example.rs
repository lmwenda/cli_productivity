
#[test]
fn if_both_numbers_add()
{
    let x: i8 = 5;
    let y: i8 = 3;

    let answer: i8 = x + y;
    let function_answer: i8 = crate::add_both_numbers(x, y);

    assert_eq!(answer, function_answer);

}


#[test]
fn if_both_numbers_not_equal()
{
    let x: i8 = 5;
    let y: i8 = 2;

    let answer: i8 = x + y;
    let function_answer: i8 = crate::add_both_numbers(5, 1);

    assert_ne!(answer, function_answer);

}

#[test]
fn if_both_numbers_sub()
{
    let x: i8 = 7;
    let y: i8 = 2;

    let answer: i8 = x - y;
    let function_answer: i8 = crate::sub_both_numbers(x, y);

    assert_eq!(answer, function_answer);
}
