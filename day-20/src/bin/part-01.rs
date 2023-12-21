use std::collections::HashMap;

fn main() {
    let input = include_str!("./test.txt");
    println!("{}", process(input));
}

#[allow(dead_code)]
#[derive(Debug, Default)]
enum Pulses {
    #[default]
    Low,
    High,
}

#[allow(dead_code)]
#[derive(Debug, Default)]
enum ModuleType {
    Button,
    FlipFlop(bool),
    #[default]
    Broadcaster,
    Conjunction(Option<HashMap<String, Pulses>>),
}

#[allow(dead_code)]
#[derive(Debug, Default)]
struct Module {
    r#type: ModuleType,
    dsm: Vec<String>,
    pulse: Pulses,
}

fn process(input: &str) -> usize {
    let mut modules_hash = HashMap::new();
    let lines = input.lines().collect::<Vec<_>>();

    for signal in lines {
        let get_dsm = |word: Vec<&str>| -> Vec<String> {
            word.last()
                .expect("msg")
                .split(',')
                .map(|elm| elm.trim().to_owned())
                .collect::<Vec<String>>()
        };
        if signal.starts_with('%') {
            let split = signal.split("->").collect::<Vec<_>>();
            let id = split[0].replace('%', "").trim().to_owned();
            let dsm = get_dsm(split);

            let module = Module {
                r#type: ModuleType::FlipFlop(false),
                dsm,
                ..Default::default()
            };
            modules_hash.insert(id, module);
        } else if signal.starts_with('&') {
            let split = signal.split("->").collect::<Vec<_>>();
            let id = split[0].replace('&', "").trim().to_owned();
            let dsm = get_dsm(split);
            let module = Module {
                r#type: ModuleType::Conjunction(None),
                dsm,
                ..Default::default()
            };
            modules_hash.insert(id, module);
        } else {
            let split = signal.split("->").collect::<Vec<_>>();
            let dsm = get_dsm(split);
            let module = Module {
                r#type: ModuleType::Broadcaster,
                dsm,
                ..Default::default()
            };
            let id = "broadcaster".to_string();
            modules_hash.insert(id, module);
        }
    }

    let broadcaster = modules_hash.get("broadcaster").expect("msg");
    let mut queue: Vec<String> = vec![];

    // queue.conca;

    for x in modules_hash {
        println!("{x:?}");
    }
    // println!("{lines:?}");
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn works() {
        let input = include_str!("./test.txt");
        assert_eq!(20, process(input));
    }
}
