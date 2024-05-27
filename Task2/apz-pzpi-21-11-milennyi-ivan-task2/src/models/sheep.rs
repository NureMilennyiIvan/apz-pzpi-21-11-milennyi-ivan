pub(crate) struct Sheep{
    id: Option<u64>,
    birth_date: u64,
    breed_id: u64,
    sex: bool,
    shepherd_id: Option<u64>,
    last_feeding_timestamp: u64,
    last_shearing_timestamp: u64
}