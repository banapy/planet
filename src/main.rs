mod planet;

use bevy::prelude::*;

fn main() {
    App::new().add_plugin(planet::PlanetPlugin).run();
}