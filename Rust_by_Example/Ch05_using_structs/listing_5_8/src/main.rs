
//Rectangle using variables
//fn main() {

//    let width1 = 30;
//    let height1 = 50;

//    println!("The area of the rectangle is {} square pixels.",
//        area(width1, height1));
//}

//fn area(width: u32, height: u32) -> u32 {
//    width * height
//}


//Rectangle using tuple
//fn main() {
//	let rect1 = (30, 50);

//    println!("The area of the rectangle is {} square pixels.",
//        area(rect1)
//    );

//}

//fn area (dementions: (u32, u32)) -> u32 {
//    dementions.0 * dementions.1
//}


//Rectangle using struct
//struct Rectangle {
//	width: u32,
//	height: u32,
//}

//fn main() {
//	let rect1 = Rectangle {
//		width: 30,
//		height: 50,

//	};
//	println!("The area of the rectangle is {} square pixels.", area(rect1));

//}

//fn area(rectangle: &Rectangle){
//	rectangle.width * rectangle.height
//}


//Using the Debug trait
//#[derive(Debug)]
//struct Rectangle {
//	width: u32,
//	height: u32,
//}

//fn main() {
//	let scale = 2;
//	let rect1 = Rectangle {
//		width: dbg!(30 * scale),
//		height: 50,
//	};

//	dbg!(&rect1);
//}


////Changing area() to a method of Rectangle
//#[derive(Debug)]
//struct Rectangle {
//	width: u32,
//	height: u32,
//}

//impl Rectangle {
//	fn area(&self) -> u32 {
//		self.width * self.height
//	}
//}
//fn main() {
//	let rect = Rectangle{
//		width: 30,
//		height: 50,
//	};
//}

//adding can_hold method ot Rectangle
#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32,
}

impl Rectangle {
	fn area(&self) -> u32 {
		self.width * self.height
	}

	fn can_hold(&self, other: &Rectangle) -> bool {
		self.width > other.width && self.height > other.height
	}

	fn square (size: u32) -> Self {
		Self {
			width: size,
			height: size,
		}
	}
}
fn main() {
	let rect1 = Rectangle{
		width: 30,
		height: 50,
	};

	let rect2 = Rectangle{
		width: 10,
		height: 40,
	};

	let rect3 = Rectangle{
		width: 60,
		height: 45,
	};

	println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
	println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

	let sq = Rectangle::square(30);
	println!("rect4 = {:#?}", sq);

}