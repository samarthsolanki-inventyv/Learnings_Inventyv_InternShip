use serde::Serialize;

#[derive(Debug, Serialize)]
struct Driver {
    name: String,
    nationality: String,
    championships_won: u32,
}

#[derive(Debug, Serialize)]
struct F1Team {
    name: String,
    base_country: String,
    championships_won: u32,
    engine_supplier: String,
    team_principal: String,
    drivers: [Driver; 2],
}

fn main(){
    let ferrari = F1Team {
        name: String::from("Ferrari"),
        base_country: String::from("Italy"),
        championships_won: 16,
        engine_supplier: String::from("Ferrari"),
        team_principal: String::from("Frédéric Vasseur"),
        drivers: [
            Driver {
                name: String::from("Charles Leclerc"),
                nationality: String::from("Monaco"),
                championships_won: 0,
            },
            Driver {
                name: String::from("Carlos Sainz"),
                nationality: String::from("Spain"),
                championships_won: 0,
            },
        ],
    };
    let json = serde_json::to_string_pretty(&ferrari).unwrap();

    println!("{}", json);




}
