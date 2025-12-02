pub fn solve() {
    let data =
        std::fs::read_to_string("input.txt").expect("test.txt should be present in the root.");

    let rotations = data.lines().collect::<Vec<_>>();
    let (mut pos, mut res) = (50, 0);

    for rot in rotations {
        let (rot_dir, rot_value) = (
            &rot[..1],
            &rot[1..].parse::<i16>().expect("Must be a valid number."),
        );

        res += rot_value / 100;
        let rot_value = rot_value % 100;

        let prev_pos = pos;
        match rot_dir {
            "L" => {
                pos -= rot_value;
                if pos <= 0 {
                    res += (prev_pos != 0) as i16;
                    pos = pos.rem_euclid(100);
                }
            }
            "R" => {
                pos += rot_value;
                if pos >= 100 {
                    res += (prev_pos != 0) as i16;
                    pos = pos.rem_euclid(100);
                }
            }
            _ => unreachable!(),
        };
    }
    println!("Res: {res}");
}
