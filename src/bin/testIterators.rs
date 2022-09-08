fn main() {
    let data = vec![1,2,3,4,5];
    let number = 10;
    let _data2: Vec<i32> = data.iter()
                        .map(|num| num*3)
                        .filter(|num| num > &number).collect();
    
    for i in _data2.iter() {
        println!("{:?}", i);
    }
    
    // We also can write this like this : 

    let _data_i: Vec<i32> = vec![1,2,3,4,5].iter()
    .map(|num| num*3)
    .filter(|num| num > &number).collect();

    for i in _data2.iter() {
        println!("{:?}", i);
    }

}