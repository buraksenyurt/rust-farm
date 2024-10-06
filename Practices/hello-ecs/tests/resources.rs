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

#[test]
fn get_score_resource_by_mutable_test() {
    let mut world = World::new();
    world.add_resource(Score(10));
    let score: &mut Score = world.get_resource_mut::<Score>().unwrap();
    score.0 += 10;
    let score = world.get_resource::<Score>().unwrap();
    assert_eq!(score.0, 20);
}

#[test]
fn add_and_get_multiple_resources_test() {
    let mut world = World::new();
    world.add_resource(Fps(30));
    world.add_resource(Score(10));
    let score: &Score = world.get_resource::<Score>().unwrap();
    assert_eq!(score.0, 10);
    let fps = world.get_resource::<Fps>().unwrap();
    assert_eq!(fps.0, 30);
}

#[derive(Debug)]
struct Score(pub u32);

impl std::ops::Deref for Score {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        &self.0
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
