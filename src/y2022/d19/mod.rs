use std::{
    cmp::Ordering,
    ops::{Add, AddAssign, DivAssign, Sub, SubAssign},
};

use regex::Regex;

use crate::utils::read_lines;

pub fn answer() {
    println!("Answer to day19: {} {}", a1(), a2());
}

#[derive(PartialEq, Debug, Default)]
struct Blueprint {
    id: f32,
    ore_robot: Resources,
    clay_robot: Resources,
    obsidian_robot: Resources,
    geode_robot: Resources,
}

//Blueprint 1: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 13 clay. Each geode robot costs 3 ore and 7 obsidian.
fn parse_blueprint(str: String) -> Blueprint {
    let re1 = Regex::new(
        "(: Each ore robot costs )|( ore. Each clay robot costs )|( ore. Each obsidian robot costs )|( ore and )|( clay. Each geode robot costs )",
    ).unwrap().replace_all(&str, " ");

    let re2 = Regex::new("(Blueprint )|( obsidian.)")
        .unwrap()
        .replace_all(&re1, "");

    let mut data = re2.split_whitespace().map(|raw| raw.parse().unwrap());

    let id = data.next().unwrap();

    let mut ore_robot = Resources::default();
    ore_robot.ore = data.next().unwrap();
    let mut clay_robot = Resources::default();
    clay_robot.ore = data.next().unwrap();
    let mut obsidian_robot = Resources::default();
    obsidian_robot.ore = data.next().unwrap();
    obsidian_robot.clay = data.next().unwrap();
    let mut geode_robot = Resources::default();
    geode_robot.ore = data.next().unwrap();
    geode_robot.obsidian = data.next().unwrap();

    Blueprint {
        id,
        ore_robot,
        clay_robot,
        obsidian_robot,
        geode_robot,
    }
}

#[derive(Debug, PartialEq, Default, Clone)]
struct Resources {
    ore: f32,
    clay: f32,
    obsidian: f32,
    geode: f32,
}

impl PartialOrd for Resources {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.clay == other.clay
            && self.geode == other.geode
            && self.obsidian == other.obsidian
            && self.ore == other.ore
        {
            return Some(Ordering::Equal);
        }

        if self.clay >= other.clay
            && self.geode >= other.geode
            && self.obsidian >= other.obsidian
            && self.ore >= other.ore
        {
            return Some(Ordering::Greater);
        }

        return Some(Ordering::Less);
    }
}

impl AddAssign<&Self> for Resources {
    fn add_assign(&mut self, rhs: &Self) {
        self.clay += rhs.clay;
        self.ore += rhs.ore;
        self.obsidian += rhs.obsidian;
        self.geode += rhs.geode;
    }
}

impl Add<Self> for &Resources {
    type Output = Resources;

    fn add(self, rhs: Self) -> Self::Output {
        let mut out = Resources::default();

        out.clay = self.clay + rhs.clay;
        out.ore = self.ore + rhs.ore;
        out.obsidian = self.obsidian + rhs.obsidian;
        out.geode = self.geode + rhs.geode;

        out
    }
}

impl DivAssign<f32> for Resources {
    fn div_assign(&mut self, by: f32) {
        self.clay /= by;
        self.ore /= by;
        self.obsidian /= by;
        self.geode /= by;
    }
}

impl SubAssign<&Self> for Resources {
    fn sub_assign(&mut self, rhs: &Self) {
        self.clay -= rhs.clay;
        self.ore -= rhs.ore;
        self.obsidian -= rhs.obsidian;
        self.geode -= rhs.geode;
    }
}

impl Sub<Self> for &Resources {
    type Output = Resources;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut out = Resources::default();

        out.clay = self.clay - rhs.clay;
        out.ore = self.ore - rhs.ore;
        out.obsidian = self.obsidian - rhs.obsidian;
        out.geode = self.geode - rhs.geode;

        out
    }
}

fn compute_quality_rec(
    minute: usize,
    resources: Resources,
    robots: Resources,
    blueprint: &Blueprint,
) -> f32 {
    if minute > 24 {
        return resources.geode;
    };

    let mut branches = vec![];

    let resources = &resources + &robots;

    let can_create_robots = resources >= blueprint.ore_robot
        || resources >= blueprint.obsidian_robot
        || resources >= blueprint.clay_robot
        || resources >= blueprint.geode_robot;

    if resources >= blueprint.ore_robot {
        let mut new_robots = robots.clone();
        new_robots.ore += 1.0;

        branches.push(compute_quality_rec(
            minute + 1,
            &resources - &blueprint.ore_robot,
            new_robots,
            blueprint,
        ));
    }
    if resources >= blueprint.obsidian_robot {
        let mut new_robots = robots.clone();
        new_robots.obsidian += 1.0;

        branches.push(compute_quality_rec(
            minute + 1,
            &resources - &blueprint.obsidian_robot,
            new_robots,
            blueprint,
        ));
    }
    if resources >= blueprint.clay_robot {
        let mut new_robots = robots.clone();
        new_robots.clay += 1.0;

        branches.push(compute_quality_rec(
            minute + 1,
            &resources - &blueprint.clay_robot,
            new_robots,
            blueprint,
        ));
    }
    if resources >= blueprint.geode_robot {
        let mut new_robots = robots.clone();
        new_robots.geode += 1.0;

        branches.push(compute_quality_rec(
            minute + 1,
            &resources - &blueprint.geode_robot,
            new_robots,
            blueprint,
        ));
    }

    // if !can_create_robots {
    branches.push(compute_quality_rec(
        minute + 1,
        resources,
        robots.clone(),
        blueprint,
    ));
    // }

    return *branches
        .iter()
        .max_by(|l, r| l.partial_cmp(r).unwrap())
        .unwrap_or(&0.0);
}

fn compute_quality(blueprint: Blueprint) -> usize {
    let resources = Resources::default();
    let mut robots = Resources::default();
    robots.ore += 1.0;

    blueprint.id as usize * compute_quality_rec(1, resources, robots, &blueprint) as usize
}

// other option: deep search
// for every minute find all possible robots to build then
// run alternative computation for every option

// ways to optimize search:
// -

fn a1() -> usize {
    read_lines("src/y2022/d19/input")
        .map(parse_blueprint)
        .map(compute_quality)
        .sum()
}

fn a2() -> usize {
    read_lines("src/y2022/d19/input");

    0
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_solve_first_problem() {
        assert_eq!(a1(), 0);
    }

    #[test]
    fn should_solve_second_problem() {
        assert_eq!(a2(), 0);
    }

    #[test]
    fn should_parse_blueprint() {
        let res = parse_blueprint("Blueprint 1: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 13 clay. Each geode robot costs 3 ore and 7 obsidian.".into());

        assert_eq!(
            res,
            Blueprint {
                id: 1.0,
                ore_robot: Resources {
                    ore: 3.0,
                    clay: 0.0,
                    obsidian: 0.0,
                    geode: 0.0
                },
                clay_robot: Resources {
                    ore: 4.0,
                    clay: 0.0,
                    obsidian: 0.0,
                    geode: 0.0
                },
                obsidian_robot: Resources {
                    ore: 4.0,
                    clay: 13.0,
                    obsidian: 0.0,
                    geode: 0.0
                },
                geode_robot: Resources {
                    ore: 3.0,
                    clay: 0.0,
                    obsidian: 7.0,
                    geode: 0.0
                }
            }
        )
    }

    #[test]
    fn should_compute_maximum_score() {
        let b = parse_blueprint("Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.".into());

        assert_eq!(compute_quality(b), 9)
    }
}
