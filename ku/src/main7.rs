use std::process::Command;
fn main(){
    let output = Command::new("rustc")
        .arg("--version")
        .output().unwrap_or_else(|e| {
            panic!("failed to execute process: {}",e)
        });

    if output.status.success(){
        let s = String::from_utf8_lossy(&output.stdout);
        print!("rustc successded and stdout was: \n{}",s);
    }else {
        let s = String::from_utf8_lossy(&output.stderr);
        print!("rustc failed and sterr was:\n{}",s);
    }
}