pub enum ObjectTypes {

}

pub enum SkinTypes {
    DefaultSkin,
    SoldierSkin
}

pub enum AccessoryTypes {
    
}

#[derive(Default)]
enum DefaultSkin {
    id = 0,
}

#[derive(Default)]
enum SoldierSkin {
    damage_multi = 0.75
}