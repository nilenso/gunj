#[derive(Debug,Clone)]
struct Dam {
    name: String,
    age: u32,
}

fn add3(dam: &mut Dam) {
    dam.age = dam.age +3;
}

fn add2(dam: &mut Dam) {
    dam.age = dam.age +3;
}

fn add1(dam: &Dam) {
    println!("dam: {:?}", dam);
}

fn main() {
    let mut dam = Dam{
        name: "Nagarjuna Sagar".to_string(),
        age: 56
    };
    // ~~~move(dam, d2);
    let d2 = dam.clone();

    println!("Hello, world!");

    add3(&mut dam);
    add2(&mut dam);
    add1(&dam);

    let d3 = dam.clone();

    dam.age = 50;
    println!("foo: {:?}", dam);
}


