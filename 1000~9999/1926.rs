// 그림

use std::collections::VecDeque;

fn main() {
    let height;
    let width;

    {
        let temp = input();

        height = temp[0];
        width  = temp[1];
    }

    let mut paper = Vec::new();

    for i in 0..height {
        paper.push(input());
        assert_eq!(paper[i].len(), width);
    }

    let mut cnt_pic = 0_u32;
    let mut max_pic_area = 0_u32;

    let mut deque = VecDeque::new();

    const MY: [i32; 5] = [0, 1, 0, -1,  0];
    const MX: [i32; 5] = [0, 0, 1,  0, -1];

    for i in 0..height {
        for j in 0..width {
            if paper[i][j] == 1 {
                cnt_pic += 1;
                deque.push_back((i, j));
            }

            let mut pic_area = 0;

            while 0 < deque.len() {
                let (cy, cx) = deque.pop_front().unwrap();

                for k in 0..5 {
                    let ny = cy as i32 + MY[k];
                    let nx = cx as i32 + MX[k];

                    if
                        0    <=    nx && nx    <    width as i32
                                      &&
                        0    <=    ny && ny    <    height as i32
                    {
                        let ny = ny as usize;
                        let nx = nx as usize;

                        if paper[ny][nx] == 1 {
                            pic_area += 1;
                            paper[ny][nx] = 0;
                            deque.push_back((ny, nx));
                        }
                    }
                }
            }

            if pic_area > max_pic_area {
                max_pic_area = pic_area;
            }
        }
    }

    println!("{}\n{}", cnt_pic, max_pic_area);
}

fn input() -> Vec<usize> {
    let mut datas = String::new();

    std::io::stdin().read_line(&mut datas).unwrap();

    datas.split_whitespace().map(|data| {
        data.trim().parse().unwrap()
    }).collect()
}
