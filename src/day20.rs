use std::io::{Read, BufRead, BufReader};
use failure::Error;
use cgmath::Vector3;
use cgmath::prelude::Zero;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Particle {
    pos: Vector3<i64>,
    vel: Vector3<i64>,
    acc: Vector3<i64>,
    destroyed: bool,
}

impl Particle {
    pub fn velocity(&self) -> u64 {
        (self.vel.x.abs() + self.vel.y.abs() + self.vel.z.abs()) as u64
    }

    pub fn acceleration(&self) -> u64 {
        (self.acc.x.abs() + self.acc.y.abs() + self.acc.z.abs()) as u64
    }
}

pub fn parse<R: Read>(reader: R) -> Result<Vec<(usize, Particle)>, Error> {
    let mut particles = Vec::new();

    for (index, line) in BufReader::new(reader).lines().enumerate() {
        let mut line = line?;
        let mut buf = &line[..];

        let mut p = Particle {
            pos: Vector3::zero(),
            vel: Vector3::zero(),
            acc: Vector3::zero(),
            destroyed: false,
        };

        while buf.len() > 0 {
            if &buf[0..2] == ", " {
                buf = &buf[2..];
            }

            let prelude = &buf[0..2];
            let rest = &buf[2..];
            let end = rest.find('>').expect("end bracket");
            let mut values = rest[1..end].split(",");

            let a = values.next().expect("a").parse::<i64>()?;
            let b = values.next().expect("b").parse::<i64>()?;
            let c = values.next().expect("c").parse::<i64>()?;

            match prelude {
                "p=" => {
                    p.pos = (a, b, c).into();
                }
                "v=" => {
                    p.vel = (a, b, c).into();
                }
                "a=" => {
                    p.acc = (a, b, c).into();
                }
                value => panic!("unexpected value: {}", value),
            }

            buf = &rest[(end + 1)..];
        }

        particles.push((index, p));
    }

    Ok(particles)
}

macro_rules! component_norm {
    ($norm:ident, $p:ident, $comp:ident) => {
        if $p.acc.$comp != 0 {
            // how much do we need to accelerate to reach the target velocity?
            let _d = $p.acc.$comp - $p.vel.$comp;
            let _n = (_d + $p.acc.$comp - 1) / $p.acc.$comp;
            $norm = i64::max($norm, _n);
        }
    }
}

pub fn part1<R: Read>(reader: R) -> Result<usize, Error> {
    let mut particles = parse(reader)?;

    // make sure all vector are moving towards their acceleration.
    let mut norm = 0i64;

    for p in particles.iter().map(|p| &p.1) {
        component_norm!(norm, p, x);
        component_norm!(norm, p, y);
        component_norm!(norm, p, z);
    }

    for p in particles.iter_mut() {
        // TODO: probably no need to iterate
        for _ in 0..norm {
            p.1.vel += p.1.acc;
            p.1.pos += p.1.vel;
        }
    }

    // lowest acceleration will always be closer in the long run.
    particles.sort_by(|a, b| a.1.acceleration().cmp(&b.1.acceleration()));

    let found = particles.first().expect("no particles").1.acceleration();
    let mut round2 = Vec::new();

    for (index, p) in particles {
        if p.acceleration() != found {
            break;
        }

        round2.push((index, p));
    }

    // lowest initial velocity will always be closer in the long run.
    round2.sort_by(|a, b| a.1.velocity().cmp(&b.1.velocity()));

    let found = round2.first().expect("no particles").1.velocity();

    let mut round3 = Vec::new();

    for (index, p) in round2 {
        if p.velocity() != found {
            break;
        }

        round3.push((index, p));
    }

    if round3.len() != 1 {
        panic!("more than one particle :(");
    }

    Ok(round3.get(0).expect("no particle").0)
}

pub fn part2<R: Read>(reader: R) -> Result<usize, Error> {
    use std::collections::hash_map::Entry::*;

    let mut particles: Vec<Particle> = parse(reader)?.into_iter().map(|(_, p)| p).collect();

    for _ in 0..1000 {
        let mut cols: HashMap<Vector3<i64>, &mut Particle> = HashMap::new();

        for p in particles.iter_mut() {
            if p.destroyed {
                continue;
            }

            p.vel += p.acc;
            p.pos += p.vel;

            match cols.entry(p.pos.clone()) {
                Vacant(e) => {
                    e.insert(p);
                }
                Occupied(mut e) => {
                    p.destroyed = true;
                    e.get_mut().destroyed = true;
                }
            }
        }
    }

    let count = particles.into_iter().filter(|p| !p.destroyed).count();
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_part1_nemesis() {
        assert_eq!(
            part1(Cursor::new(
                "p=<0,0,0>, v=<0,0,0>, a=<1,0,0>\np=<0,0,0>, v=<-1,0,0>, a=<1,0,0>\n",
            )).unwrap(),
            1
        );
    }
}

const INPUT: &str = include_str!("../input/day20.txt");

problem!{
    tests => [
        run_part1 => {part1(::std::io::Cursor::new(INPUT)), "4e03b4fe2fb42aee01ba79601d20b0c3321ef87bedc89b43bc023b4d989983d0"},
        run_part2 => {part2(::std::io::Cursor::new(INPUT)), "365fb0f62e2a989bbd2f649bbe706528fd973479aaddc42b3fe9bf08d73d05ab"},
    ];
}
