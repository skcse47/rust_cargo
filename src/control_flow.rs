
pub fn flow(){
    let mut counter = 0;

    let y = loop {

        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("Value of counter is {y}");
}

pub fn nested_loop(){
    let mut count = 0;

    'counting: loop{
        println!("count = {count}");
        let mut remaining = 10;
        loop{
            println!("remaining = {remaining}");
            if remaining == 9{
                break;
            }

            if count == 2{
                break 'counting;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("Last count = {count}");
}