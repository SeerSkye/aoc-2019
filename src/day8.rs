use std::fs;

pub fn day_8() {
    let layer_size = 25 * 6;

    let input = fs::read_to_string("input/day8.txt").expect("Could not read file!");

    assert!(input.len() % layer_size == 0); //Let's just make sure this holds...

    let mut layers: Vec<&str> = Vec::new();

    //this is a little gross
    for i in 0..input.len() / layer_size {
        layers.push(&input[i * layer_size..(i + 1) * layer_size]);
    }

    let target_layer: &str = layers
        .iter()
        .min_by(|x, y| {
            x.chars()
                .filter(|c| *c == '0')
                .count()
                .cmp(&y.chars().filter(|c| *c == '0').count())
        })
        .unwrap();
    let num_1s = target_layer.chars().filter(|c| *c == '1').count();
    let num_2s = target_layer.chars().filter(|c| *c == '2').count();

    println!("Num 1s * Num 2s is: {}", num_1s * num_2s);

    let mut image: String = layers.pop().unwrap().to_string();

    while !layers.is_empty() {
        image = image
            .chars()
            .zip(layers.pop().unwrap().chars())
            .map(|(ichar, lchar)| if lchar == '2' { ichar } else { lchar })
            .collect();
    }

    //this is also kind of gross
    for i in 0..layer_size / 25 {
        println!("{}", &image[i*25..(i+1)*25])
    }
}
