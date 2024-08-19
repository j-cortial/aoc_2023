use std::collections::{HashMap, HashSet, VecDeque};

use gcd::Gcd;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline, one_of},
    combinator::{map, opt},
    multi::separated_list1,
    sequence::{pair, separated_pair},
};
use strum::{EnumCount, EnumIs};

type ModuleId = &'static str;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIs)]
enum ModuleKind {
    Broadcast,
    FlipFlop,
    Conjunction,
}

#[derive(Debug, PartialEq, Eq)]
struct Module {
    kind: ModuleKind,
    destinations: Vec<ModuleId>,
}

#[derive(Debug)]
struct Network {
    modules: HashMap<ModuleId, Module>,
}

impl Network {
    fn sources(&self, module_id: ModuleId) -> Vec<ModuleId> {
        self.modules
            .iter()
            .filter(|&(_id, module)| module.destinations.contains(&module_id))
            .map(|(id, _module)| *id)
            .collect()
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, EnumCount, EnumIs)]
enum Energy {
    #[default]
    Low,
    High,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum FlipFlopState {
    #[default]
    Off,
    On,
}

impl FlipFlopState {
    fn flip(self) -> (Self, Energy) {
        match self {
            FlipFlopState::Off => (FlipFlopState::On, Energy::High),
            FlipFlopState::On => (FlipFlopState::Off, Energy::Low),
        }
    }
}

#[derive(Debug)]
struct NetworkState {
    flipflop_states: HashMap<ModuleId, FlipFlopState>,
    conjunction_states: HashMap<ModuleId, HashMap<ModuleId, Energy>>,
    probe: Option<ModuleId>,
}

impl NetworkState {
    fn new(network: &Network, probe: Option<ModuleId>) -> Self {
        let flipflop_states = network
            .modules
            .iter()
            .filter_map(|(&id, module)| {
                (module.kind.is_flip_flop()).then_some((id, Default::default()))
            })
            .collect();
        let conjunction_states = network
            .modules
            .iter()
            .filter_map(|(&id, module)| {
                (module.kind.is_conjunction()).then_some((
                    id,
                    network
                        .modules
                        .iter()
                        .filter_map(|(&source_id, module)| {
                            module
                                .destinations
                                .contains(&id)
                                .then_some((source_id, Default::default()))
                        })
                        .collect(),
                ))
            })
            .collect();
        Self {
            flipflop_states,
            conjunction_states,
            probe,
        }
    }
}

#[derive(Debug)]
struct NetworkActivity<'a> {
    network: &'a Network,
    state: NetworkState,
    counts: [usize; Energy::COUNT],
    probe_signal: Vec<(ModuleId, Energy)>,
}

impl<'a> NetworkActivity<'a> {
    fn new(network: &'a Network, probe: Option<ModuleId>) -> Self {
        Self {
            network,
            state: NetworkState::new(network, probe),
            counts: Default::default(),
            probe_signal: Default::default(),
        }
    }

    fn pulse_count(&self, energy: Energy) -> usize {
        self.counts[energy as usize]
    }

    fn increment_count(&mut self, energy: Energy) {
        self.counts[energy as usize] += 1;
    }

    fn press_button(&mut self) {
        self.probe_signal.clear();
        let mut pulses: VecDeque<(ModuleId, ModuleId, Energy)> = VecDeque::new();
        pulses.push_back(("button", "broadcaster", Energy::Low));
        while let Some((source_id, target_id, energy)) = pulses.pop_front() {
            self.increment_count(energy);
            if self.state.probe == Some(target_id) {
                self.probe_signal.push((source_id, energy));
            }
            if let Some(target_module) = self.network.modules.get(target_id) {
                match target_module.kind {
                    ModuleKind::Broadcast => {
                        for id in &target_module.destinations {
                            pulses.push_back((target_id, id, energy));
                        }
                    }
                    ModuleKind::FlipFlop => match energy {
                        Energy::Low => {
                            let old_state = self.state.flipflop_states.get_mut(target_id).unwrap();
                            let (new_state, new_energy) = old_state.flip();
                            for id in &target_module.destinations {
                                pulses.push_back((target_id, id, new_energy));
                            }
                            *old_state = new_state;
                        }
                        Energy::High => {}
                    },
                    ModuleKind::Conjunction => {
                        let state = self.state.conjunction_states.get_mut(target_id).unwrap();
                        *state.get_mut(source_id).unwrap() = energy;
                        let new_energy = if state.values().all(|energy| energy.is_high()) {
                            Energy::Low
                        } else {
                            Energy::High
                        };
                        for id in &target_module.destinations {
                            pulses.push_back((target_id, id, new_energy));
                        }
                    }
                }
            }
        }
    }
}

fn parse_input(input: &'static str) -> Network {
    map(
        separated_list1(
            newline::<&str, ()>,
            map(
                separated_pair(
                    pair(opt(one_of("&%")), alpha1),
                    tag(" -> "),
                    separated_list1(tag(", "), alpha1),
                ),
                |((sigil, id), destinations)| {
                    let kind = match sigil {
                        Some(c) => match c {
                            '%' => ModuleKind::FlipFlop,
                            '&' => ModuleKind::Conjunction,
                            _ => panic!(),
                        },
                        None => ModuleKind::Broadcast,
                    };
                    let module = Module { kind, destinations };
                    (id, module)
                },
            ),
        ),
        |entries| Network {
            modules: entries.into_iter().collect(),
        },
    )(input)
    .unwrap()
    .1
}

fn solve_part1(network: &Network) -> usize {
    let mut activity = NetworkActivity::new(network, None);
    for _ in 0..1000 {
        activity.press_button();
    }
    activity.pulse_count(Energy::Low) * activity.pulse_count(Energy::High)
}

fn solve_part2(network: &Network) -> usize {
    let rx_sources = network.sources("rx");
    assert_eq!(rx_sources.len(), 1);
    let rx_source = rx_sources[0];
    assert_eq!(
        network.modules.get(rx_source).map(|t| t.kind),
        Some(ModuleKind::Conjunction)
    );
    let mut activity = NetworkActivity::new(network, Some(rx_source));
    let generators: HashSet<_> = activity.state.conjunction_states.get(rx_source).map(|s| s.keys().copied().collect()).unwrap();
    let mut history: HashMap<_, Vec<_>> = HashMap::new();
    for i in 0.. {
        activity.press_button();
        let high_pulses: Vec<_> = activity
            .probe_signal
            .iter()
            .filter(|(_, e)| e.is_high())
            .map(|(id, _)| id)
            .collect();
        let change = !high_pulses.is_empty();
        for &p in high_pulses {
            match history.entry(p) {
                std::collections::hash_map::Entry::Occupied(mut entry) => entry.get_mut().push(i),
                std::collections::hash_map::Entry::Vacant(entry) => {
                    entry.insert(vec![i]);
                }
            };
        }
        if change {
            if history.len() == generators.len() && history.values().all(|v| v.len() >= 2) {
                return history.values().map(|v| v[v.len() - 1] - v[v.len() - 2]).reduce(lcm).unwrap();
            }
        }
    }
    0
}

fn lcm(first: usize, second: usize) -> usize {
    first * second / first.gcd(second)
}

fn main() {
    let input = include_str!("../../data/day20.txt");
    let network = parse_input(input);
    let answer1 = solve_part1(&network);
    println!("The answer to part 1 is {}", answer1);
    let answer2 = solve_part2(&network);
    println!("The answer to part 2 is {}", answer2);
}

#[cfg(test)]
mod test {
    use crate::{parse_input, solve_part1};

    const INPUT: &str = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&parse_input(INPUT)), 32_000_000);
    }
}
