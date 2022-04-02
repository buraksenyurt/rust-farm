fn main() {

}

pub enum State{
    Online,
    OutOfService,
    OnTheMove(Location), // Bonus :) Rust enum veri yapısının zenginliğini kullandık. OnTheMove state'indeyken örneğin lokasyonunu da tutalım dedik.
    Destroyed
}

pub struct Location{
    pub x:f32,
    pub y:f32
}


