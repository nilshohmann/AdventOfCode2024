use crate::riddles::Riddle;
use crate::riddles::utils::Utils;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct MemorySpace {
    id: u32,
    size: u32,
    empty: bool,
}

pub struct Day09();

impl Riddle for Day09 {
    fn day(&self) -> u8 { 9 }

    fn validate_first(&self) -> bool {
        Utils::verify(self._solve_first("input_test.txt"), 1928)
    }

    fn solve_first(&self) -> String {
        self._solve_first("input.txt").to_string()
    }

    fn validate_second(&self) -> bool {
        Utils::verify(self._solve_second("input_test.txt"), 2858)
    }

    fn solve_second(&self) -> String {
        self._solve_second("input.txt").to_string()
    }
}

impl Day09 {
    fn _solve_first(&self, filename: &str) -> u64 {
        let mut memory = self.read_memory(filename);

        loop {
            let space_index = Self::find_first_empty_space(&memory);
            let file_index = Self::find_last_taken_space(&memory);

            let space = memory[space_index].clone();
            let file = memory[file_index].clone();

            if space_index > file_index {
                break;
            }

            if space.size <= file.size {
                memory[space_index].id = file.id;
                memory[space_index].empty = false;

                if space.size < file.size {
                    memory[file_index].size -= space.size;
                    memory.insert(file_index + 1, MemorySpace { id: 0, size: file.size - space.size, empty: true });
                } else {
                    memory[file_index].empty = true;
                }
            } else {
                memory[space_index] = file.clone();
                memory.insert(space_index + 1, MemorySpace { id: 0, size: space.size - file.size, empty: true });
                memory.remove(file_index + 1);
            }
        }

        Self::checksum(&memory)
    }

    fn _solve_second(&self, filename: &str) -> u64 {
        let mut memory = self.read_memory(filename);
        let mut space_id = memory[Self::find_last_taken_space(&memory)].id;

        loop {
            let file_index = Self::find_by_id(&memory, space_id);
            let file = memory[file_index].clone();

            if let Some(space_index) = Self::find_first_empty_space_fitting(&memory, file.size) {
                if space_index < file_index {
                    let space = memory[space_index].clone();

                    memory[space_index] = file.clone();
                    memory[file_index].empty = true;

                    if space.size > file.size {
                        memory.insert(space_index + 1, MemorySpace {
                            id: 0,
                            size: space.size - file.size,
                            empty: true,
                        });
                    }
                }
            }

            if space_id == 0 {
                break;
            }
            space_id -= 1;
        }

        Self::checksum(&memory)
    }

    fn read_memory(&self, filename: &str) -> Vec<MemorySpace> {
        let mut is_empty = false;
        let mut id: u32 = 0;

        let mut memory: Vec<MemorySpace> = Vec::new();

        for c in self.read_input_file(filename).chars() {
            let size = c.to_digit(10).unwrap();

            if is_empty {
                memory.push(MemorySpace { id: 0, size, empty: true })
            } else {
                memory.push(MemorySpace { id, size, empty: false });
                id += 1;
            }

            is_empty = !is_empty;
        }

        memory
    }

    fn find_first_empty_space(memory: &Vec<MemorySpace>) -> usize {
        memory.iter().enumerate().filter(|(_, m)| m.empty).map(|(i, _)| i).next().unwrap()
    }

    fn find_first_empty_space_fitting(memory: &Vec<MemorySpace>, min_size: u32) -> Option<usize> {
        memory.iter().enumerate().filter(|(_, m)| m.empty && m.size >= min_size).map(|(i, _)| i).next()
    }

    fn find_last_taken_space(memory: &Vec<MemorySpace>) -> usize {
        memory.iter().enumerate().filter(|(_, m)| !m.empty).last().map(|(i, _)| i).unwrap()
    }

    fn find_by_id(memory: &Vec<MemorySpace>, id: u32) -> usize {
        memory.iter().enumerate().filter(|(_, m)| !m.empty && m.id == id).map(|(i, _)| i).last().unwrap()
    }

    fn checksum(memory: &Vec<MemorySpace>) -> u64 {
        let mut result = 0u64;

        let mut i = 0u64;
        for m in memory {
            if m.empty {
                i += m.size as u64;
                continue;
            }

            for _ in 0..m.size {
                result += i * (m.id as u64);
                i += 1;
            }
        }

        result
    }
}
