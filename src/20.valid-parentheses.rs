/*
 * @lc app=leetcode id=20 lang=rust
 *
 * [20] Valid Parentheses
 */

// @lc code=start
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];
        for c in s.chars() {
            match c {
                '(' | '{' | '[' => stack.push(c),
                ')' | '}' | ']' => match stack.last() {
                    Some(maybe_closing_paren) => match get_matching_char(c) {
                        Ok(closing_paren) => {
                            if *maybe_closing_paren == closing_paren {
                                stack.pop();
                            } else {
                                return false;
                            }
                        }
                        Err(_) => return false,
                    },
                    None => return false,
                },
                _ => return false,
            }
        }

        return stack.is_empty();
    }
}

fn get_matching_char(c: char) -> Result<char, ()> {
    match c {
        ')' => Ok('('),
        '}' => Ok('{'),
        ']' => Ok('['),
        _ => Err(()),
    }
}
// @lc code=end
