pub fn part1(input: &str) -> String {
    let input: Vec<u32> = input.chars().filter_map(|c| c.to_digit(10)).collect();
    let blocks: Vec<String> = build_blocks(input);
    let blocks = move_blocks(blocks);

    checksum(blocks).to_string()
}

fn build_blocks(input: Vec<u32>) -> Vec<String> {
    let mut blocks: Vec<String> = Vec::new();
    for (i, num) in input.into_iter().enumerate() {
        if num == 0 {
            continue;
        } else if i % 2 == 0 {
            for _ in 0..num {
                blocks.push(((i / 2) as u32).to_string());
            }
        } else {
            for _ in 0..num {
                blocks.push(".".to_string());
            }
        }
    }

    blocks
}

fn checksum(blocks: Vec<String>) -> u64 {
    let mut checksum = 0;

    for (id, block) in blocks.into_iter().enumerate() {
        if let Ok(b) = block.parse::<u64>() {
            checksum += id as u64 * b;
        }
    }

    checksum
}

fn move_blocks(blocks: Vec<String>) -> Vec<String> {
    let mut reader = blocks.len() - 1;
    let mut writer = 0;
    let mut result = blocks.clone();

    while writer < reader {
        while writer < reader && result[writer] != ".".to_string() {
            writer += 1;
        }
        while reader > writer && result[reader] == ".".to_string() {
            reader -= 1;
        }

        result[writer] = result[reader].clone();
        result[reader] = ".".to_string();
        reader -= 1;
        writer += 1;
    }

    result
}

fn build_blocks_part_2(input: Vec<u32>) -> (Vec<String>, Vec<(u32, String, usize)>) {
    let mut blocks: Vec<String> = Vec::new();
    let mut block_info: Vec<(u32, String, usize)> = Vec::new();

    for (i, num) in input.into_iter().enumerate() {
        if num == 0 {
            continue;
        } else if i % 2 == 0 {
            block_info.push((num, ((i / 2) as u32).to_string(), blocks.len()));
            for _ in 0..num {
                blocks.push(((i / 2) as u32).to_string());
            }
        } else {
            block_info.push((num, ".".to_string(), blocks.len()));
            for _ in 0..num {
                blocks.push(".".to_string());
            }
        }
    }

    (blocks, block_info)
}

fn find_space(result: &Vec<String>, freq: usize, block_index: usize) -> Option<usize> {
    let mut count = 0;
    for i in 0..block_index {
        if result[i] == "." {
            count += 1;
            if count == freq {
                let start = i + 1 - freq;
                return Some(start);
            }
        } else {
            count = 0;
        }
    }
    None
}

fn move_blocks_part_2(blocks: Vec<String>, block_info: Vec<(u32, String, usize)>) -> Vec<String> {
    let mut result = blocks.clone();

    for i in (0..block_info.len()).rev() {
        let (freq, content, block_index) = block_info[i].clone();
        if content == "." {
            continue;
        }

        if let Some(space_start) = find_space(&result, freq as usize, block_index) {
            for j in block_index..block_index + freq as usize {
                result[j] = ".".to_string();
            }
            for j in space_start..space_start + freq as usize {
                result[j] = content.clone();
            }
        }
    }

    result
}

pub fn part2(input: &str) -> String {
    let input: Vec<u32> = input.chars().filter_map(|c| c.to_digit(10)).collect();
    let (blocks, block_info) = build_blocks_part_2(input);
    let blocks = move_blocks_part_2(blocks, block_info);

    checksum(blocks).to_string()
}
