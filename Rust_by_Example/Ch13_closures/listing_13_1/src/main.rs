
#[derive(Debug, PartialEq, Copy, Clone)]
enum Shirt_Color {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<Shirt_Color>,
}

impl Inventory {
    fn givaway(
        &self,
        user_preference: Option<Shirt_Color>
    ) -> Shirt_Color {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> Shirt_Color {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                Shirt_Color::Red => num_red += 1,
                Shirt_Color::Blue => num_blue +=1,
            }
        }
        if num_red > num_blue {
            Shirt_Color::Red
        } else {
            Shirt_Color::Blue
        }
    }
}

fn main() {
    let store = Inventory{
        shirts: vec![
            Shirt_Color::Blue,
            Shirt_Color::Red,
            Shirt_Color::Blue,
        ],
    };

    let user_pref1 = Some(Shirt_Color::Red);
    let giveaway1 = store.givaway(user_pref1);
    println!("The user with preference {:?} gets {:?}",
    user_pref1, giveaway1);


    let user_pref2 = None;
    let giveaway2 = store.givaway(user_pref2);
    println!("The user with preference {:?} gets {:?}",
    user_pref2, giveaway2);
}
