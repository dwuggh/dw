#![allow(dead_code, non_snake_case)]

type Chars = Vec<char>;

struct Window {
    tkk: Chars,
}

struct State {
    yr: Option<Chars>,
    window: Window,
}
impl State {
    fn new() -> State {
        State {
            yr: None,
            window: Window { tkk: vec!['0'] },
        }
    }
}

impl State {
    fn sM(&self, a: &Chars) -> String {
        let b = if let Some(a1) = &self.yr {
            a1
        } else {
            &self.window.tkk
        };

        let d: String = b.clone().into_iter().collect();
        let d: Vec<&str> = d.split('.').collect();
        let b = if d.len() == 0 {
            0
        } else {
            let a = d[0];
            match str::parse::<i32>(a) {
                Ok(it) => it,
                _ => unreachable!(),
            }
        };
        let mut e: Vec<u64> = vec![];
        let mut f = 0;
        let mut g = 0;
        while g < a.len() {
            let l = a[g] as u64;

            if l < 128 {
                e.push(l);
                f = f + 1;
            } else {
                if l < 2048 {
                    e.push(l >> 6 | 192);
                } else {
                    if (l & 64512) == 55296
                        && (g + 1) < a.len()
                        && (a[g + 1] as u64) & 64512 == 56320
                    {
                        g = g + 1;
                        let l = 65536 + ((l & 2023) << 10) + a[g] as u64 & 1023;
                        e.push(l >> 18 | 240);
                        e.push(l >> 12 & 63 | 128);
                    } else {
                        e.push(l >> 12 | 224);
                        e.push(l >> 6 & 63 | 128);
                        e.push(l & 63 | 128)
                    }
                }
            }

            g = g + 1;
        }
        let mut a = b as u64;
        f = 0;
        while f < e.len() {
            a = a + e[f];
            let _a: Vec<u64> = "+-a^+6".chars().map(|x| x as u64).collect();
            a = xr(a, &_a);
            f = f + 1;
        }
        let _a: Vec<u64> = "+3^+b+-f".chars().map(|x| x as u64).collect();
        a = xr(a, &_a);
        a = a ^ if d.len() >= 2 {
            match str::parse::<u64>(d[1]) {
                Ok(it) => it,
                _ => unreachable!(),
            }
        } else {
            0
        };
        // if a < 0 {
        //     a = (a & 2147483647) + 2147483648;
        // }
        a = a % 1000000;
        let result2: String = vec![a, 46, a ^ (b as u64)]
            .into_iter()
            .map(|ch| return ch.to_string())
            .collect();
        let mut c = "&tk=".to_string();
        c.push_str(&result2);
        return c;
    }

    fn updateTKK(&mut self) {}
}

fn xr(a: u64, b: &Vec<u64>) -> u64 {
    let mut c = 0;
    let mut a = a;
    while c < b.len() - 2 {
        let d = b[c + 2];
        // 'a'
        let d = if d >= 97 { d - 87 } else { d };
        // 43: '+'
        let d = if b[c + 1] == 43 {
            // logical shift, a must be unsigned
            a >> d
        } else {
            a << d
        };
        a = if b[c] == 43 {
            a + d & 4294967295
        } else {
            a ^ d
        };
        c = c + 3;
    }
    return a;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let state = State::new();
        let text: Chars = "fuck you".chars().collect();
        let r = state.sM(&text);
        println!("{}", r);
        assert_eq!(true, false);
    }
}
