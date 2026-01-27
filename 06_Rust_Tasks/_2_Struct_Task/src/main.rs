#[derive(Debug)]
struct Driver {
    name: String,
    nationality: String,
    championships_won: u32,
}

#[derive(Debug)]
struct F1Team {
    name: String,
    base_country: String,
    championships_won: u32,
    engine_supplier: String,
    team_principal: String,
    drivers: [Driver; 2], // Fixed-size array
}

impl F1Team {
    // Getters
    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_base_country(&self) -> &String {
        &self.base_country
    }

    fn get_championships_won(&self) -> u32 {
        self.championships_won
    }

    fn get_engine_supplier(&self) -> &String {
        &self.engine_supplier
    }

    fn get_team_principal(&self) -> &String {
        &self.team_principal
    }

    fn get_team(&self) -> String {
        format!(
            "Team: {}, Base: {}, Championships: {}, Engine: {}, Principal: {}",
            self.name,
            self.base_country,
            self.championships_won,
            self.engine_supplier,
            self.team_principal
        )
    }

    // Setters
    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn set_base_country(&mut self, base_country: String) {
        self.base_country = base_country;
    }

    fn set_championships_won(&mut self, championships_won: u32) {
        self.championships_won = championships_won;
    }

    fn set_engine_supplier(&mut self, engine_supplier: String) {
        self.engine_supplier = engine_supplier;
    }

    fn set_team_principal(&mut self, team_principal: String) {
        self.team_principal = team_principal;
    }
    fn set_driver_championships(&mut self, index: usize, championships: u32) {
        self.drivers[index].championships_won = championships;
    }

    fn set_driver_name(&mut self, index: usize, name: String) {
        self.drivers[index].name = name;
    }

    fn set_driver_nationality(&mut self, index: usize, nationality: String) {
        self.drivers[index].nationality = nationality;
    }
}

fn main() {
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

    println!("{}", ferrari.get_team());
   

    println!("Drivers:");
    for (index, driver) in ferrari.drivers.iter().enumerate() {
        println!(
            "Driver {}: {} | Nationality: {} | Championships: {}",
            index + 1,
            driver.name,
            driver.nationality,
            driver.championships_won
        );
    }
}
