use std::fs;


fn number_of_tree_part_1(biome:Vec<String>,right:usize,down:usize)->usize{
    let n:usize =biome.get(0).expect("Need at least one string").len();
    assert!(n>3,"Not implemented");

    let mut it =biome.iter();
    it.next();
    let mut tree_count =0;
    let mut pos:usize=0;
    while it.len()>0{
        pos+=right;
        pos%=n;
        for i in 0..down - 1 {
            it.next();
        }
        let current=it.next().expect("Missing it");
        if current.chars().nth(pos as usize ).expect("Index modulo wrong")=='#'{
            tree_count+=1;
        }
    }
    tree_count
}


fn number_of_tree_part_2(biome:Vec<String>)->usize{
    let n1=number_of_tree_part_1(biome.clone(),1,1);
    let n2=number_of_tree_part_1(biome.clone(),3,1);
    let n3=number_of_tree_part_1(biome.clone(),5,1);
    let n4=number_of_tree_part_1(biome.clone(),7,1);
    let n5=number_of_tree_part_1(biome.clone(),1,2);
    println!("{} {} {} {} {}", n1,n2,n3,n4,n5);
    n1*n2*n3*n4*n5
}
fn main() {
    let input = fs::read_to_string("input/test.txt")
        .expect("Something went wrong reading the file");
    let lines = input.lines();
    let mut biome: Vec<String> = vec![];
    for line in lines {
        biome.push(line.parse::<String>().expect("Ouf that's not a string !"))
    }
    println!("{}",number_of_tree_part_1(biome.clone(),3,1));
    println!("{}",number_of_tree_part_2(biome.clone()));
}
