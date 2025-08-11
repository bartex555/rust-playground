mod vec2d;
use vec2d::Vec2D;

fn main() {
    //not the ideal way to test because of float imprecision 
    let a: Vec2D = Vec2D{x:2.0,y: 0.0};
    let b: Vec2D =  Vec2D{x:2.0,y: 0.0};
    let c: Vec2D = a + b;
    assert!(c == Vec2D{x:4.0,y:0.0});
    let d: Vec2D = c.normalize();
    assert!(d == Vec2D{x:1.0,y:0.0});
    let e = d - Vec2D{x:8.0,y:2.0};
    assert!(e == Vec2D{x:-7.0,y:-2.0});

    println!("{}",e);
}
