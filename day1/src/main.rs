use std::fs;
use std::cmp::min;


fn main() {
    let input = fs::read_to_string("input/test.txt")
        .expect("Something went wrong reading the file");
    let mut lines = input.lines();
    let mut numbers =vec![];
    for line in lines{
        numbers.push(line.parse::<u16>().expect("Ouf that's not a number !"))
    }
    // this is the key part
    numbers.sort();
    let (mut low,high):(Vec<u16>, Vec<u16>)=numbers.iter().partition(|&&x| x<=(2020/2));
    let mut n:usize =min(low.len(), high.len());
    if n==0 {
        assert!(low.len()>=2);
        let last_low:u16=*low.last().expect("Something was wrong, there should be an index here");
        low.pop();
        let last_last_low:u16=*low.last().expect("Something was wrong, there should be an index here");
        assert_eq!(last_last_low, last_low);
        assert_eq!(last_last_low, 1024);
        println!("Found 1024 twice ! {}",last_last_low*last_low);
    }
    while n > 0 {
        let last_low:u16=*low.last().expect("Something was wrong, there should be an index here");
        low.pop();
        for i in 0..high.len() {
            let current_high:u16=*high.get(i).expect("Something was wrong, there should be an index here");
            if last_low+current_high==2020{
                println!("Found {} and {} which makes {}",last_low,current_high,last_low*current_high);
            }
            if last_low+current_high>2020{
                // we know that this low is not that good so we can discard it
                break;
            }
        }
        n-=1;
    }
}
