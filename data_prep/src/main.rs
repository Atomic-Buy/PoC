
use data_prep::*; 
fn main (){
    let path = "./src.jpg"; 
    // build a Data15K object from the image
    let data15k = Data15K::new(path).unwrap();
    // export the data15k as a json file
    data15k.export_json("./prove1.json").unwrap();
    // check if the json is aligned with the data15k
    assert!(data15k.check_json("./prove1.json"));
}