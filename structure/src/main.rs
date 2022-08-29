#[derive(Debug)]
struct Rectangle
{
    lenght : u32,
    width: u32
}

fn main() {
    let scale : u32 = 2;
    let rect1 = Rectangle{ lenght: 50, width: dbg!(30 * scale) };
    dbg!(&rect1);
}