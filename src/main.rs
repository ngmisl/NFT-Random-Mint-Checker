use rand::Rng;

// TODO: take user input
// TODO: Simulate x-times output average

fn random_minter(users: i32, ran: i32, days: i32) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut counter = 0;
    let mut specials: Vec<i32> = vec![];

    for _x in 0..users * days {
        // simulate claims
        let num: i32 = rng.gen_range(0..ran); // range 1 - 999
        if num <= 995 {
            //    dbg!("{}", num);
        } else {
            // dbg!("Special: {}", num);
            counter += 1;
            specials.push(num);
        }
    }
    return vec![counter, users, days];
}

fn main() {
    let minter_return = random_minter(1000, 1000, 30);
    println!(
        "Final: {:?} rare mints, with {:?} daily users in {:?} days",
        minter_return[0], minter_return[1], minter_return[2]
    );
}
