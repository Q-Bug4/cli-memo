use climemo::exec_python_script;

fn main() {
    let output = exec_python_script("/home/kael/tmp/test.py");
    println!("{output}")
}