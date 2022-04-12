enum HockeyPosition{
    Wing
}

struct HockeyPlayer{
    name: String,
    number: u8,
    position: HockeyPosition,
    goals_ytd:u8 
}

fn main() {
    // Intialize a mutable instance
    let mut player = HockeyPlayer{
        name: String::from("Weiren Lan"),
        number: 18,
        position: HockeyPosition::Wing,
        goals_ytd: 8
    };

    //We could use dot to use the field 
    player.goals_ytd +=1;
    println!(
        "{} has scored {} goals this season",
        player.name,
        player.goals_ytd
    );
}
