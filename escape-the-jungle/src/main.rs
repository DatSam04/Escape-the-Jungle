use std::io;

/* 
    Player will wake up in the middle of jungle, and they need to find a way to
    escape before getting killed by wild animals in the jungle.
*/

fn main() {
    let mut alive = true;
    let mut scene = 0;
    let mut answer: i32;
    while alive {
        if scene == 0 {
            println!("\nYou feel stressed during work, so you went to the jungle for fresh air. Unfortunately, while looking for a \
            new plant, you get lost in the middle of the jungle. Suddenly, a tree fall on your back, and you lost \
            conscious. When you wake up, it is already dark. Then, you hear a wild animal voice, what will you do next?");
            scene = 1;
        }else if scene == 1 {
            scene1();
            println!("Enter your answer: ");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("failed to read line");
            answer = input.trim().parse().unwrap();
            println!("player choice: {}\n", answer);
            if answer != 1 {
                alive = false;
                if answer == 2 {
                    println!("You were killed by a bear");
                }else{
                    println!("You were killed by a panther");
                }
            }else {
                scene = 2;
                println!("Running toward the river, you can still heard the movement behind. \
                        You is now at the river, but the sound is getting closure, what should you do next?");
            }
        }else if scene == 2 {
            scene2();
            println!("Enter your answer: ");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("failed to read line");
            answer = input.trim().parse().unwrap();
            println!("player choice: {}\n", answer);
            if answer != 2 {
                alive = false;
                if answer == 1 {
                    println!("With the bear swimming skills, it catches up and kill you");
                }else if answer == 3 {
                    println!("The food cans you bring were opened during the run, so the bear smell it and find your hide");
                }
            }else {
                scene = 3;
                println!("The bear can't catch up with you in the ground, so it was left behind. \
                Not long after, you find a house with light on, and you also find a small path nearby. \
                What will you do next?");
            }
        }else if scene == 3 {
            scene3();
            println!("Enter your answer: ");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("failed to read line");
            answer = input.trim().parse().unwrap();
            println!("player choice: {}\n", answer);
            if answer != 2 {
                alive = false;
                if answer == 1 {
                    println!("There is a group of psycho party inside the house. When \
                    they see you, they all express their disgusting behavior and characteristics. \
                    They catch and torture you for a long time.");
                }else if answer == 3 {
                    println!("A wolf herd is drinking ahead, when they notice you, they chase and catch up with you.");
                }
            }else {
                scene = 4;
                println!("The bear hasn't stopped chasing yet, while you stop to take a break and choose the next path, \
                the bear catch up with you, and you run toward the small path next to the house. \
                After a short distance, you find a bag, but there is a leopard nearby. \
                What should you do next?");
            }
        }else if scene == 4 {
            scene4();
            println!("Enter your answer: ");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("failed to read line");
            answer = input.trim().parse().unwrap();
            println!("player choice: {}\n", answer);
            if answer != 3 {
                alive = false;
                if answer == 1 {
                    println!("You through the rock and hit the leopard eye, which make it angry. \
                    The leopard attack you fiercely");
                }else if answer == 2 {
                    println!("You get injured heavily while fighting with the leopard, \
                    then it jump over you, and you hold your branch high up, and the \
                    branch pierce through the leopard heart. However, it's still able \
                    to hit your neck, and you lost a lot of blood.");
                }
            }else {
                scene = 5;
                println!("After running awhile, the bag is dragging you down, so you through \
                it away. A knife, a lighter, and a torch fall out of the bag. \
                What will you do next?");
            }
        }else if scene == 5 {
            scene5();
            println!("Enter your answer: ");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("failed to read line");
            answer = input.trim().parse().unwrap();
            println!("player choice: {}\n", answer);
            if answer != 3 {
                alive = false;
                if answer == 1 {
                    println!("On a head to head fight, you use all the force to stab \
                    the knife to the leopard heart and kill it. Unfortunately, the \
                    bear appear behind it and attack you fiercely.");
                } else if answer == 2 {
                    println!("You light up the torch and attack the leopard, but the \
                    damage is too weak, so it only make the leopard more angry. It \
                    attack back and injured you heavily.");
                }
            }else {
                println!("The village always have a guard in the border tower that \
                next to the jungle to keep watch of wild animals. Fortunately, before the leopard catch up with you, the guard see you running, and he kill \
                it with a sniper. You are save and return to the city.");
                println!("You are victory!\n");
                break;
            }
        }
        if alive == false {
            println!("You dead!!!\n");
            println!("Do you want to play again (1 for yes and 0 for no)?");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("failed to read line");
            answer = input.trim().parse().unwrap();
            if answer == 1 {
                alive = true;
                scene = 0;
            }
        }
    }
    println!("Thank you for playing");
}

// scene 1 where the player just wake up
fn scene1() {
    let mut option = String::new();
    option.push_str("\n1. Go to the river.\n");
    option.push_str("2. Go to the jungle.\n");
    option.push_str("3. Hide above a tree.\n");
    println!("{}", option);
}

// scene 2 where the player arrive at the river
fn scene2() {
    let mut option = String::new();
    option.push_str("\n1. Swim in the river\n");
    option.push_str("2. Run along the river.\n");
    option.push_str("3. Hide in a bush near the river.\n");
    println!("{}", option);
}

// scene 3 where the player arrive infront of the house
fn scene3() {
    let mut option = String::new();
    option.push_str("\n1. Go into the house.\n");
    option.push_str("2. Follow the path.\n");
    option.push_str("3. Keep walking along the river.\n");
    println!("{}", option);
}

//scene 4 where the play follow the path next to the house
fn scene4() {
    let mut option = String::new();
    option.push_str("\n1. Use a Rock.\n");
    option.push_str("2. Use a branch with sharp head.\n");
    option.push_str("3. Run around\n.");
    println!("{}", option);
}

// scene 5 where the player through the bag and see a lot of weapon
fn scene5() {
    let mut option = String::new();
    option.push_str("\n1. Use the knife and fight back.\n");
    option.push_str("2. Light up the torch to defense.\n");
    option.push_str("3. Keep running toward the village.\n");
    println!("{}", option);
}