enum Access {
    Privilege,
    Guest,
    Owner
}


fn main() {
    let access_level = Access::Privilege;
    let can_access = match access_level {
        Access::Owner => true,
        _ => false,
    };

    println!("can access this ? {:?}",can_access);
}