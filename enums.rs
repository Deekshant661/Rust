//makes the code more strict

//we can also use strings here but that will make our code loose.
enum Direction{
    North,
    South,
    East,
    West,
}

fn main(){
    let my_Direction = Direction::North;
    moves_around(my_Direction);
}

fn moves_around(direction: Direction){
    println!("should print something");
    //some logic
}
