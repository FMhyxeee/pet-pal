use pet_pal::AnimationSystem;

fn main() {
    let aim = AnimationSystem::load_from("img/Stocking");

    println!("animation_system.name: {aim:?}");
}
