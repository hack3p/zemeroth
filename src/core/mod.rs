use std::default::Default;
use std::collections::{hash_map, HashMap};
use core::map::{HexMap, PosHex};
use core::movement::MovePoints;

pub use core::execute::execute;
pub use core::check::check;

pub mod command;
pub mod event;
pub mod movement;
pub mod effect;
pub mod map;
pub mod execute;

#[macro_use]
pub mod component_storage;

mod check;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PlayerId(pub i32); // TODO: make field private

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ObjId(i32);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Strength(pub i32);

#[derive(Clone, Debug)]
pub struct UnitType {
    pub name: String,
    pub attack_distance: map::Distance,
    pub move_points: MovePoints,
    pub moves: Moves,
    pub attacks: Attacks,
    pub jokers: Jokers,
    pub reactive_attacks: Attacks,
    pub strength: Strength,
}

#[derive(Clone, Debug)]
pub struct Unit {
    pub pos: PosHex,
    pub player_id: PlayerId,
    pub moves: Moves,
    pub attacks: Attacks,
    pub jokers: Jokers,
    pub strength: Strength,
    pub unit_type: UnitType,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Attacks(pub i32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Moves(pub i32);

/// Move or Attack
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Jokers(pub i32);

#[derive(Clone, Copy, Debug)]
pub enum TileType {
    Floor,
    Lava,
}

impl Default for TileType {
    fn default() -> Self {
        TileType::Floor
    }
}

#[derive(Clone, Debug)]
pub struct ObjIdIter<'a> {
    iter: hash_map::Keys<'a, ObjId, Unit>,
}

impl<'a> Iterator for ObjIdIter<'a> {
    type Item = ObjId;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().cloned()
    }
}

// -----------------------------------------------

pub mod component {
    #[derive(Clone, Debug)]
    pub struct Move {
        pub move_points: i32,
    }

    #[derive(Clone, Debug)]
    pub struct Health {
        pub points: i32,
    }

    #[derive(Clone, Debug)]
    pub struct Attack {
        pub active_points: i32,
    }
}

// TODO: How can I represent UnitTypes? Can I use them with other types of objects?

// TODO: How can I implement object_ids_at? ObjId iterator + filter?

make_storage!(Storage<ObjId>: {
    health: component::Health,
    movement: component::Move,
    attack: component::Attack,
    unit: Unit,
});

// -----------------------------------------------

#[derive(Clone, Debug)]
pub struct State {
    units: HashMap<ObjId, Unit>, // TODO: replace with `Storage`
    map: HexMap<TileType>,
    next_obj_id: ObjId,
    player_id: PlayerId,
    players_count: i32,
}

impl State {
    pub fn new() -> Self {
        {
            let mut state = Storage::new();
            println!("state: {:#?}", state);
            let id = ObjId(0);
            println!("initital: is_exist = {}", state.is_exist(id));
            let health = component::Health { points: 3 };
            let attack = component::Attack { active_points: 1 };
            state.health.insert(id, health);
            println!("added: is_exist = {}", state.is_exist(id));
            println!("added: {:?}", state.health.get(id));
            state.health.get_mut(id).points = 7;
            println!("updated: {:?}", state.health.get(id));
            state.attack.insert(id, attack);
            state.debug_print_entity(id);
            println!("state: {:#?}", state);
            state.health.remove(id);
            println!("removed: is_exist: {}", state.is_exist(id));
        }

        let radius = map::Distance(5); // TODO: pass `Options` struct
        let mut map = HexMap::new(radius);
        {
            // TODO: load\generate maps
            map.set_tile(PosHex { q: 0, r: 0 }, TileType::Lava);
        }
        let units = HashMap::new();
        let next_obj_id = ObjId(0);
        Self {
            units,
            map,
            next_obj_id,
            player_id: PlayerId(0),
            players_count: 2,
        }
    }

    pub fn player_id(&self) -> PlayerId {
        self.player_id
    }

    pub fn alloc_id(&mut self) -> ObjId {
        let id = self.next_obj_id;
        self.next_obj_id.0 += 1;
        id
    }

    pub fn map(&self) -> &HexMap<TileType> {
        &self.map
    }

    pub fn obj_iter(&self) -> ObjIdIter {
        ObjIdIter {
            iter: self.units.keys(),
        }
    }

    pub fn unit_opt(&self, id: ObjId) -> Option<&Unit> {
        self.units.get(&id)
    }

    pub fn unit(&self, id: ObjId) -> &Unit {
        self.unit_opt(id).unwrap()
    }

    pub fn units_at(&self, pos: PosHex) -> Vec<&Unit> {
        let mut units_at = Vec::new();
        for unit in self.units.values() {
            if unit.pos == pos {
                units_at.push(unit);
            }
        }
        units_at
    }

    pub fn object_ids_at(&self, pos: PosHex) -> Vec<ObjId> {
        let mut ids = Vec::new();
        for (&id, unit) in &self.units {
            if unit.pos == pos {
                ids.push(id);
            }
        }
        ids
    }
}
