/*
 * @lc app=leetcode id=67 lang=rust
 *
 * [67] Add Binary
 */

use crate::util::solution::Solution;

// @lc code=start
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let a: Vec<char> = a.chars().rev().collect();
        let b: Vec<char> = b.chars().rev().collect();
        let sum = match add_binary_helper(a.as_slice(), b.as_slice(), false) {
            Ok(sum) => sum,
            Err(_) => return String::new(),
        };
        let sum: String = sum.iter().rev().collect();
        return sum;
    }
}

fn add_binary_helper(a: &[char], b: &[char], carry: bool) -> Result<Vec<char>, ()> {
    let sum = match (&a[..], &b[..]) {
        ([], []) => {
            if carry {
                vec!['1']
            } else {
                vec![]
            }
        }
        (n @ [n_char, n_rest @ ..], []) | ([], n @ [n_char, n_rest @ ..]) => {
            if carry {
                let (digit, new_cary) = match n_char {
                    '0' => ('1', false),
                    '1' => ('0', true),
                    _ => return Err(()),
                };

                let mut result = vec![digit];
                let mut rest_result = match add_binary_helper(n_rest, vec![].as_slice(), new_cary) {
                    Ok(v) => v,
                    Err(_) => return Err(()),
                };
                result.append(&mut rest_result);
                result
            } else {
                n.to_vec()
            }
        }
        ([a_char, a_rest @ ..], [b_char, b_rest @ ..]) => {
            let (digit, new_carry) = match (a_char, b_char) {
                ('0', '0') => (if carry { '1' } else { '0' }, false),
                ('1', '1') => (if carry { '1' } else { '0' }, true),
                ('0', '1') | ('1', '0') => {
                    if carry {
                        ('0', true)
                    } else {
                        ('1', false)
                    }
                }
                _ => return Err(()),
            };
            let mut rest_result = match add_binary_helper(a_rest, b_rest, new_carry) {
                Ok(v) => v,
                Err(_) => return Err(()),
            };
            let mut result = vec![digit];
            result.append(&mut rest_result);
            result
        }
    };
    return Ok(sum);
}
// @lc code=end
