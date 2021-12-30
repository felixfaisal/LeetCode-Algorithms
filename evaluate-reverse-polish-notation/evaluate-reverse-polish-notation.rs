impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = vec![];
        
        for x in tokens{
            match &x[..] {
                "+" => {
                    let op1 = stack[stack.len()-1];
                    stack.pop(); 
                    let op2 = stack[stack.len()-1];
                    stack.pop();
                    let result= op1+op2;
                    stack.push(result);
                }
                "-" => {
                    let op1 = stack[stack.len()-1];
                    stack.pop(); 
                    let op2 = stack[stack.len()-1];
                    stack.pop();
                    let result= op2-op1;
                    stack.push(result);
                }
                "*" => {
                    let op1 = stack[stack.len()-1];
                    stack.pop(); 
                    let op2 = stack[stack.len()-1];
                    stack.pop();
                    let result= op1*op2;
                    stack.push(result);
                }
                "/" => {
                    let op1 = stack[stack.len()-1];
                    stack.pop(); 
                    let op2 = stack[stack.len()-1];
                    stack.pop();
                    let result= op2/op1;
                    stack.push(result);
                }
                _ => {
                    stack.push(x.parse().unwrap());
                }
            }
        }
        stack[stack.len()-1]        
    }
}