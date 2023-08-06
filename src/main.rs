use bevy::prelude::*;
use rand::prelude::*;
use std::io;
use std::collections::HashMap;
use std::process;

const NUM_PLAYERS: usize = 7;

#[derive(Debug, PartialEq)]
pub enum Role {
    Doctor,
    Detective,
    Mafia,
    Townsperson,
}

#[derive(Debug)]
pub struct Player {
    name: String,
    role: Role,
}

fn get_player_index(name: &String, players: &Vec<Player>) -> usize {
    for (index, player) in players.iter().enumerate() {
        if name == &player.name {
            return index;
        }
    }

    0
}

mod day_time {
    use super::*;

    pub fn vote(players: &Vec<Player>) -> usize {
        let mut player_response = String::new();
        let mut vote_map = HashMap::new();
        println!("Voting begins.");

        for player in players {
            println!("{} is voting", player.name);
            player_response = get_player_response();
            let index = get_player_index(&player_response, players);
            let votes = vote_map.entry(index).or_insert(0);
            *votes += 1;
        }
        
        let mut voted = 0;
        let mut biggest = 0;
        for (index, votes) in &vote_map {
            if *votes > biggest {
                biggest = *votes;
                voted = *index;
            }
        }
        
        voted
    }
}

mod night_time {
    use super::*;

    pub fn night_time_logic(
        players: &Vec<super::Player>,
        detective_response: &mut String,
        doctor_response: &mut String,
        mafia_response: &mut String) 
        {
        println!("lights off.");

        for player in players {
            match player.role {
                Role::Doctor => {
                    println!("{} - Select player to save.", player.name);
                    *doctor_response = get_player_response();
                }
                Role::Detective => {
                    println!("{} - Select player to investigate.", player.name);
                    *detective_response = get_player_response();

                    let mut suspect: &Player = &Player {
                        name: String::from("A"), 
                        role: Role::Townsperson,
                    };

                    for player in players {
                        if *detective_response == player.name {
                            suspect = player;
                        }
                    }

                    match suspect.role {
                        Role::Doctor => {
                            println!("{} is a doctor", suspect.name );
                        }
                        Role::Mafia => {
                            println!("{} is a mafia", suspect.name);
                        }
                        Role::Townsperson => {
                            println!("{} is a townsperson", suspect.name);
                        }
                        _ => {}
                    }
                }
                Role::Mafia => {
                    println!("{} - Select target.", player.name);
                    *mafia_response = super::get_player_response();
                }
                Role::Townsperson => {
                    println!("{} - Zzz", player.name);
                }
            }
        }
    }
}

fn main() {
    App::new()
        .add_systems(Update, run_mafia_game)
        .run();
}

fn run_mafia_game() {
    println!("Welcome to Mafia Game by SaucyTuna!");
    let mut players = create_players(NUM_PLAYERS);

    let mut doctor_response = String::new();
    let mut detective_response = String::new();
    let mut mafia_response = String::new();

    let mut is_night: bool = true;
    loop {
        if is_night {
            println!("{:?}", players);

            night_time::night_time_logic(
                &players,
                &mut detective_response,
                &mut doctor_response,
                &mut mafia_response
            );

            if doctor_response == mafia_response {
                println!("The doctor saved Player {}.", doctor_response);
            } else {
                println!("The mafia shot Player {}.", mafia_response);
                let target_index = get_player_index(&mafia_response, &mut players);
                players.remove(target_index);
            }

            is_night = false;
        } else /* daytime */ {
            println!("{:?}", players);
            let voted_index = day_time::vote(&players);
            let voted = players.get(voted_index).unwrap(); 
            println!("The citizens voted {}", voted.name);
            players.remove(voted_index);
            check_situation(&players);
            is_night = true;
        }
    }
}

fn check_situation(players: &Vec<Player>) {
    for player in players {
        if player.role == Role::Mafia {
            return;
        }
    }

    println!("Villagers win!");
    process::exit(0);
}

fn create_players(n: usize) -> Vec<Player> {
    // create roles and assign them to players
    let mut players: Vec<Player> = Vec::new();


    let mut roles = Vec::new();
    roles.push(Role::Doctor);
    roles.push(Role::Detective);
    roles.push(Role::Mafia);
    roles.push(Role::Townsperson);
    roles.push(Role::Townsperson);
    roles.push(Role::Townsperson);
    roles.push(Role::Townsperson);

    {
        let mut rng = rand::thread_rng();
        roles.shuffle(&mut rng);
    }

    for i in 0..n {
        let player = create_player(
            format!("Player {}", i+1),
            roles.pop().unwrap_or_else(|| Role::Townsperson)
        );

        println!("{}'s role is {:?}.", player.name, player.role);
        players.push(player);
    }

    players
}

fn create_player(name: String, role: Role) -> Player {
    Player { name, role }
}

fn get_player_response() -> String {
    let mut response = String::new();
    io::stdin()
        .read_line(&mut response)
        .expect("Failed to read line.");

    response.trim().parse().expect("Please enter a valid name.")
}


