use bevy::{ecs::schedule::ScheduleLabel, prelude::*, window::PrimaryWindow};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PeoplePlugin)
        .run();
}

// region:			--- EP01

struct PeoplePlugin;

impl Plugin for PeoplePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, print_names)
            .add_systems(Update, people_with_jobs)
            .add_systems(Update, people_without_jobs)
            .add_systems(Update, people_does_job);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Person {
            name: "Tony".to_string(),
        },
        Employed { job: Job::Doctor },
    ));
    commands.spawn(Person {
        name: "Zack".to_string(),
    });
    commands.spawn((
        Person {
            name: "Bender".to_string(),
        },
        Employed { job: Job::Lawyer },
    ));
}

fn print_names(person_query: Query<&Person>) {
    for person in person_query.iter() {
        println!("Name: {}", person.name);
    }
}

fn people_with_jobs(person_query: Query<&Person, With<Employed>>) {
    for people in person_query.iter() {
        println!("{} has a job.", people.name);
    }
}
fn people_without_jobs(person_query: Query<&Person, Without<Employed>>) {
    for people in person_query.iter() {
        println!("{} is ready for hire", people.name);
    }
}

fn people_does_job(person_query: Query<(&Person, &Employed)>) {
    for (people, employed) in person_query.iter() {
        println!("{} does {:?}", people.name, employed.job);
    }
}

#[derive(Component)]
struct Person {
    pub name: String,
}

#[derive(Component)]
struct Employed {
    job: Job,
}

#[derive(Debug)]
pub enum Job {
    Doctor,
    FireFighter,
    Lawyer,
}
