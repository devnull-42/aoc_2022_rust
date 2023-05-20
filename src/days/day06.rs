pub fn part1(input: &str) -> String {
    let mut packet = Vec::<char>::new();

    for (idx, c) in input.chars().enumerate() {
        let found = find_char(&packet, c);

        if let Some(i) = found {
            packet.drain(0..=i);
            packet.push(c);
        } else {
            if packet.len() == 3 {
                packet.push(c);
                return (idx + 1).to_string();
            } else {
                packet.push(c);
            }
        }
    }

    "".to_string()
}

pub fn part2(input: &str) -> String {
    let mut packet = Vec::<char>::new();

    for (idx, c) in input.chars().enumerate() {
        let found = find_char(&packet, c);

        if let Some(i) = found {
            packet.drain(0..=i);
            packet.push(c);
        } else {
            if packet.len() == 13 {
                packet.push(c);
                return (idx + 1).to_string();
            } else {
                packet.push(c);
            }
        }
    }

    "".to_string()
}

fn find_char(vec: &Vec<char>, ch: char) -> Option<usize> {
    for (i, &c) in vec.iter().enumerate() {
        if c == ch {
            return Some(i);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_a() {
        const INPUT: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let result = part1(INPUT);
        assert_eq!(result, "7");
    }

    #[test]
    fn test_part1_b() {
        const INPUT: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let result = part1(INPUT);
        assert_eq!(result, "5");
    }

    #[test]
    fn test_part1_c() {
        const INPUT: &str = "nppdvjthqldpwncqszvftbrmjlhg";
        let result = part1(INPUT);
        assert_eq!(result, "6");
    }

    #[test]
    fn test_part1_d() {
        const INPUT: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let result = part1(INPUT);
        assert_eq!(result, "10");
    }

    #[test]
    fn test_part1_e() {
        const INPUT: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        let result = part1(INPUT);
        assert_eq!(result, "11");
    }

    #[test]
    fn test_part2_a() {
        const INPUT: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let result = part2(INPUT);
        assert_eq!(result, "19");
    }

    #[test]
    fn test_part2_b() {
        const INPUT: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let result = part2(INPUT);
        assert_eq!(result, "23");
    }

    #[test]
    fn test_part2_c() {
        const INPUT: &str = "nppdvjthqldpwncqszvftbrmjlhg";
        let result = part2(INPUT);
        assert_eq!(result, "23");
    }

    #[test]
    fn test_part2_d() {
        const INPUT: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let result = part2(INPUT);
        assert_eq!(result, "29");
    }

    #[test]
    fn test_part2_e() {
        const INPUT: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        let result = part2(INPUT);
        assert_eq!(result, "26");
    }
}
