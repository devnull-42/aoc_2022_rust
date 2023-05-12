pub fn part1(input: &str) -> String {
    let mut overlap_count: u32 = 0;
    input.lines().for_each(|line| {
        let workers: Vec<&str> = line.split(",").collect();
        let sections: Vec<&str> = workers[0].split("-").chain(workers[1].split("-")).collect();

        let worker1_start = sections[0].parse::<u32>().unwrap();
        let worker1_end = sections[1].parse::<u32>().unwrap();
        let worker2_start = sections[2].parse::<u32>().unwrap();
        let worker2_end = sections[3].parse::<u32>().unwrap();

        if worker1_start <= worker2_start && worker1_end >= worker2_end {
            overlap_count += 1;
        } else if worker2_start <= worker1_start && worker2_end >= worker1_end {
            overlap_count += 1;
        }
    });

    overlap_count.to_string()
}

pub fn part2(input: &str) -> String {
    let mut overlap_count: u32 = 0;
    input.lines().for_each(|line| {
        let workers: Vec<&str> = line.split(",").collect();
        let sections: Vec<&str> = workers[0].split("-").chain(workers[1].split("-")).collect();

        let worker1_start = sections[0].parse::<u32>().unwrap();
        let worker1_end = sections[1].parse::<u32>().unwrap();
        let worker2_start = sections[2].parse::<u32>().unwrap();
        let worker2_end = sections[3].parse::<u32>().unwrap();

        if worker1_end >= worker2_start && worker1_start <= worker2_end {
            overlap_count += 1;
        }
    });

    overlap_count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_part1() {
        let result = part1(INPUT);
        assert_eq!(result, "2");
    }

    #[test]
    fn test_part2() {
        let result = part2(INPUT);
        assert_eq!(result, "4");
    }
}
