use hello_ecs::World;

#[test]
fn create_fps_resource_test() {
    let mut world = World::new();
    world.add_resource(Fps(30));
    dbg!(&world);
    if let Some(fps) = world.get_resource::<Fps>() {
        assert_eq!(fps.0, 30);
    }
}

#[derive(Debug)]
struct Fps(pub u32);

impl std::ops::Deref for Fps {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
