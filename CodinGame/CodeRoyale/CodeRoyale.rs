use std::io;
use std::collections::HashMap;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/

#[derive(Debug)]
struct Site {
    x: i32,
    y: i32,
    radius: i32,
    structure_type: i32,
    owner: i32,
    cooldown: i32,
    unit: i32
}

impl Site {
    fn new(x: i32, y: i32, radius: i32) -> Site {
        Site {
            x,
            y,
            radius,
            structure_type: -1,
            owner: -1,
            cooldown: -1,
            unit: -1
        }
    }

    pub fn update(&mut self, struct_type: i32, owner: i32,
                  cooldown: i32, unit: i32) {
        self.structure_type = struct_type;
        self.owner = owner;
        self.cooldown = cooldown;
        self.unit = unit;
    }
}

#[derive(Debug)]
struct Unit {
    x: i32,
    y: i32,
    owner: i32,
    utype: i32,
    health: i32
}

impl Unit {
    fn new(x: i32, y: i32, owner: i32, utype: i32, health: i32) -> Unit {
        Unit {
            x,
            y,
            owner,
            utype,
            health
        }
    }

    pub fn is_queen(&self) -> bool {
        self.utype == -1
    }

    pub fn get_location(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

fn queen_location(units: &Vec<Unit>) -> (i32, i32) {
    for u in units {
        if u.is_queen() {
            return u.get_location()
        }
    }

    (-1, -1)
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let num_sites = parse_input!(input_line, i32);

    let mut sites: HashMap<i32, Site> = HashMap::new();

    for i in 0..num_sites as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let site_id = parse_input!(inputs[0], i32);
        let x = parse_input!(inputs[1], i32);
        let y = parse_input!(inputs[2], i32);
        let radius = parse_input!(inputs[3], i32);

        sites.insert(site_id, Site::new(x, y, radius));
    }

    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let gold = parse_input!(inputs[0], i32);
        let touched_site = parse_input!(inputs[1], i32); // -1 if none

        for i in 0..num_sites as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let site_id = parse_input!(inputs[0], i32);
            let ignore_1 = parse_input!(inputs[1], i32); // used in future leagues
            let ignore_2 = parse_input!(inputs[2], i32); // used in future leagues
            let structure_type = parse_input!(inputs[3], i32); // -1 = No structure, 2 = Barracks
            let owner = parse_input!(inputs[4], i32); // -1 = No structure, 0 = Friendly, 1 = Enemy
            let param_1 = parse_input!(inputs[5], i32);
            let param_2 = parse_input!(inputs[6], i32);

            sites.get_mut(&site_id).unwrap().
                update(structure_type, owner, param_1, param_2);
        }

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let num_units = parse_input!(input_line, i32);

        let mut units = vec![];

        for i in 0..num_units as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let x = parse_input!(inputs[0], i32);
            let y = parse_input!(inputs[1], i32);
            let owner = parse_input!(inputs[2], i32);
            let unit_type = parse_input!(inputs[3], i32); // -1 = QUEEN, 0 = KNIGHT, 1 = ARCHER
            let health = parse_input!(inputs[4], i32);

            units.push(Unit::new(x, y, owner, unit_type, health));
        }

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        eprintln!("Gold: {}", gold);
        eprintln!("Touched site: {}", touched_site);

        eprintln!("Structures:");
        for s in &sites {
            eprintln!("{:?}", s);
        }

        eprintln!("Num units: {}", num_units);
        for u in &units {
            eprintln!("{:?}", u);
        }
        eprintln!("Queen on: {:?}", queen_location(&units));


        // First line: A valid queen action
        // Second line: A set of training instructions
        println!("WAIT");
        println!("TRAIN");
    }
}