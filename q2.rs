fn get_sum(x:Vec<u32>)->Option<u32>{
    let mut result:u32 = 0;
    for i in &x{
        match result.checked_add(*i) {
            None => {
                println!("overflow!");}
            _    => {
                result = result + i;}
        }
    }
        Some(result)
}

fn main(){
    let number_list = [35,50,15,120,60];
    let res = get_sum((&number_list).to_vec());
    println!("question2：");
    println!("numbers:{:?}",number_list);
    println!("sum:{:?}",res);
}
