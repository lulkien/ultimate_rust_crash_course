fn main() {
    const STARTING_MISSILES: i32 = 8;
    const READY_AMOUNT: i32 = 2;
    // const (STARTING_MISSILES, READY_AMOUNT): (i32, i32) = (8, 2);   // we CANNOT do that

    let mut missiles: i32 = STARTING_MISSILES;
    let ready: i32 = READY_AMOUNT;
    // let (mut missiles, ready): (i32, i32) = (STARTING_MISSILES, READY_AMOUNT);  // we CAN do that
    println!("Firing {} of my {} missiles...", ready, missiles);

    missiles = missiles - ready;
    println!("{} missiles left", missiles);
}
