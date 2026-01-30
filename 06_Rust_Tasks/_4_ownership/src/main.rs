#[derive(Debug, Clone)]
struct Driver {
    name: String,
    nationality: String,
    championships_won: u32,
}

#[derive(Debug, Clone)]
struct F1Team {
    name: String,
    base_country: String,
    championships_won: u32,
    engine_supplier: String,
    team_principal: String,
    drivers: [Driver; 2],
}

impl F1Team {
    fn print(&self) {
        println!("{}", self.team_details(&self));
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
        self.print();
    }

    fn set_base_country(&mut self, base_country: String) {
        self.base_country = base_country;
        self.print();
    }

    fn set_championships_won(&mut self, championships_won: u32) {
        self.championships_won = championships_won;
        self.print();
    }

    fn set_engine_supplier(&mut self, engine_supplier: String) {
        self.engine_supplier = engine_supplier;
        self.print();
    }

    fn set_team_principal(&mut self, team_principal: String) {
        self.team_principal = team_principal;
        self.print();
    }

    fn set_driver_name(&mut self, index: usize, name: String) {
        self.drivers[index].name = name;
        self.print();
    }

    fn set_driver_nationality(&mut self, index: usize, nationality: String) {
        self.drivers[index].nationality = nationality;
        self.print();
    }

    fn set_driver_championships(&mut self, index: usize, championships: u32) {
        self.drivers[index].championships_won = championships;
        self.print();
    }

    fn team_details(&self, team: &F1Team) -> String {
        format!(
            "Team1 : {:p}

Name : {}
Base Country : {}
Championships Won : {}
Engine Supplier : {}
Team Principal : {}

Driver 1 : {} ({}, {})
Driver 2 : {} ({}, {})

Team2 : {:p}

Name : {}
Base Country : {}
Championships Won : {}
Engine Supplier : {}
Team Principal : {}

Driver 1 : {} ({}, {})
Driver 2 : {} ({}, {})",
            self,
            self.name,
            self.base_country,
            self.championships_won,
            self.engine_supplier,
            self.team_principal,
            self.drivers[0].name,
            self.drivers[0].nationality,
            self.drivers[0].championships_won,
            self.drivers[1].name,
            self.drivers[1].nationality,
            self.drivers[1].championships_won,
            team,
            team.name,
            team.base_country,
            team.championships_won,
            team.engine_supplier,
            team.team_principal,
            team.drivers[0].name,
            team.drivers[0].nationality,
            team.drivers[0].championships_won,
            team.drivers[1].name,
            team.drivers[1].nationality,
            team.drivers[1].championships_won,
        )
    }
}

fn main() {
    let mut red_bull = F1Team {
        name: "Red Bull Racing".to_string(),
        base_country: "United Kingdom".to_string(),
        championships_won: 6,
        engine_supplier: "Honda RBPT".to_string(),
        team_principal: "Christian Horner".to_string(),
        drivers: [
            Driver {
                name: "Max Verstappen".to_string(),
                nationality: "Dutch".to_string(),
                championships_won: 4,
            },
            Driver {
                name: "Sergio Pérez".to_string(),
                nationality: "Mexican".to_string(),
                championships_won: 0,
            },
        ],
    };

    let maclaren = &mut red_bull;

    maclaren.set_name("McLaren".to_string());
  //  maclaren.set_base_country("United Kingdom".to_string());
   // maclaren.set_championships_won(1);
   // maclaren.set_engine_supplier("Mercedes".to_string());
   // maclaren.set_team_principal("Andrea Stella".to_string());

   // maclaren.set_driver_name(0, "Lando Norris".to_string());
   // maclaren.set_driver_nationality(0, "British".to_string());
   // maclaren.set_driver_championships(0, 0);

   // maclaren.set_driver_name(1, "Oscar Piastri".to_string());
   // maclaren.set_driver_nationality(1, "Australian".to_string());
   // maclaren.set_driver_championships(1, 0);

   // println!("Team via ref : {:p}", maclaren);
   // println!("Team original: {:p}", &red_bull);
}
