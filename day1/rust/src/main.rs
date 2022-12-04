use std::io;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut rolling_buffer: i32 = 0;
    let mut max_calorie = MaxCalorie {
        elf_counter: 0,
        elf: 0,
        calorie: 0,
    };

    loop {
        match stdin.read_line(&mut buffer) {
            Ok(_) => {
                if buffer.starts_with::<char>('\n') {
                    //print!("empty {}\n", rolling_buffer);
                    max_calorie.add(rolling_buffer);
                    rolling_buffer = 0;
                } else {
                    match buffer.trim().parse::<i32>() {
                        Ok(data) => {
                            rolling_buffer += data;
                            //print!("roll {}\n", rolling_buffer);
                        }
                        Err(_) => break,
                    }
                }
                buffer.clear();
            }
            Err(_) => break,
        }
    }

    print!("elf: {}, cal {}\n", max_calorie.elf, max_calorie.calorie);
    Ok(())
}

struct MaxCalorie {
    elf_counter: i32,
    elf: i32,
    calorie: i32,
}

impl MaxCalorie {
    fn add(&mut self, number: i32) {
        self.elf_counter += 1;
        if number > self.calorie {
            self.calorie = number;
            self.elf = self.elf_counter;
        }
    }
}
