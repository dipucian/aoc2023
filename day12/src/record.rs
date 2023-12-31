use std::str::from_utf8;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Record {
    pub slots: Vec<u8>,
    pub counts: Vec<usize>,
}
impl Record {
    pub fn from_str(line: &str) -> Self {
        let parts = line.split_ascii_whitespace().collect::<Vec<_>>();
        Record {
            slots: parts[0].bytes().collect(),
            counts: parts[1].split(',').map(|s| s.parse().unwrap()).collect(),
        }
    }

    pub fn unfold(mut self) -> Self {
        self.counts = self.counts.repeat(5);

        self.slots.push(b'?');
        self.slots = self.slots.repeat(5);
        self.slots.pop();

        self
    }
}

impl std::fmt::Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let slots = from_utf8(&self.slots).unwrap();
        let counts = self.counts.iter().map(|&n| n.to_string()).collect::<Vec<_>>().join(",");
        write!(f, "{} {}", slots, counts)
    }
}