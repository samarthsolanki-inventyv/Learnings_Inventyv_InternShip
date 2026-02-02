pub fn Start_Execute() {
    let mut count = 1;
    let result = loop {
        count += 1;
        if count == 5{
            break count * 2;
        }
    };
    println!("Return Value using break in an Infinite Loop: {}",result);
    count = 0;
    while count < 10{
        count += 1;
        if count == 4 || count == 6 || count == 8 || count == 9 {
            continue;
        }
        println!("Inside While Loop only Primes: {}",count);
    }
    let mut i = 1;

    'outer: loop {
        println!("Outer loop: i = {}", i);

        if i > 3 {
            break 'outer;  
        }

        let mut j = 1;

        'inner: loop {
            println!("  Inner loop: j = {}", j);

            if j >= 3 {
                break 'inner;  
            }

            j += 1;
        }

        i += 1;
    }
  i = 0;
    for i in 1..=count{
        if count % i == 0 {continue};
        println!("Inside the for loop{}",i);
    } 
       
}



