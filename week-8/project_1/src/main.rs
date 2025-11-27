use std::io;

fn main() {
    let aps_table: Vec<(&str, &str, u32, u32)> = vec![
        ("intern", "APS 1-2", 0, 2),
        ("paralegal placement", "APS 1-2", 0, 2),
        ("administrator", "APS 3-5", 3, 5),
        ("research assistant", "APS 3-5", 3, 5),
        ("junior associate", "APS 3-5", 3, 5),
        ("classroom associate", "APS 3-5", 3, 5),
        ("senior administrator", "APS 5-8", 5, 8),
        ("phd candidate", "APS 5-8", 5, 8),
        ("associate lawyer", "APS 5-8", 5, 8),
        ("snr teacher", "APS 5-8", 5, 8),
        ("office manager", "EL1 8-10", 8, 10),
        ("post-doc researcher", "EL1 8-10", 8, 10),
        ("senior associate 1-2", "EL1 8-10", 8, 10),
        ("leading teacher", "EL1 8-10", 8, 10),
        ("director", "EL2 10-13", 10, 13),
        ("senior lecturer", "EL2 10-13", 10, 13),
        ("senior associate 3-4", "EL2 10-13", 10, 13),
        ("deputy principal", "EL2 10-13", 10, 13),
        ("ceo", "SES", 14, 50),
        ("dean", "SES", 14, 50),
        ("partner", "SES", 14, 50),
        ("principal", "SES", 14, 50),
    ];
    println!("Enter your current job title (e.g., Associate Lawyer):");
    let mut title_input = String::new();
    io::stdin().read_line(&mut title_input)
        .expect("Failed to read line");
    let user_title = title_input.trim().to_lowercase();

    println!("Enter your years of work experience (e.g., 6):");
    let mut exp_input = String::new();
    io::stdin().read_line(&mut exp_input)
        .expect("Failed to read line");
    let user_experience: u32 = match exp_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid experience entered. Exiting program.");
            return;
        }
    };
     let mut found= false;
     for (role,aps_level,min_exp,max_exp) in &aps_table{
    if user_title == *role && user_experience>= *min_exp && user_experience <=*max_exp{
        println!("The staff holds position {}",aps_level );
        found=true;
        break;
        
    }
    
    
}
if !found {
        println!("invalid details");
    }



}