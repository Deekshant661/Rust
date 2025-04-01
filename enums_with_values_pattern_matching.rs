//Enums with values
// Pattern Matching in Enums

enum Shape {
    Circle(f64),
    Square(f64),
    Rectangle(f64,f64),
}

fn area(shape: Shape) -> f64{
    let ans = match shape{
        Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
        Shape::Square(side) => side*side,
        Shape::Rectangle(width,height) => width*height,
    };
    return ans;
}

fn main(){
    let circle = Shape::Circle(4_f64);
    println!("Area of shape circle is : {}",area(circle));
}
