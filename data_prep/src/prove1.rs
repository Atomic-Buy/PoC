
use num_traits::Zero;
use rand::Rng;
use std::io::prelude::*;
use num_bigint::BigUint;

#[derive(Clone, Copy)]
pub struct Num{
    pub data: [u8; 30], 
}

impl Num {
    pub fn new_rand() -> Num{
        let mut rng = rand::thread_rng();
        let data: Vec<u8> = (0..30).map(|_| rng.gen()).collect();
        let mut data_arr: [u8; 30] = [0; 30];
        // fill data_arr by data 
        for i in 0..30{
            data_arr[i] = data[i];
        }
        Num{data: data_arr}
        
    }
    pub fn new_zero() -> Num {
        Num{data: [0; 30]}
    
    }
    pub fn to_decimal(&self) -> String{
        // consider the 30 bytes as a 240 bits number, the number is concatenate by the bytes from 0 to 29
        // write the big number in decimal format
        let data = self.data;
        // convert the data to a BigUint using big endian
        let big_num = BigUint::from_bytes_be(&data);
        big_num.to_str_radix(10)
    }
    pub fn parse_decimal(s: &str) -> Num{
        // parse the string to a BigUint big endian 
        let big_num = BigUint::parse_bytes(s.as_bytes(), 10).unwrap();
        // convert the BigUint to a 30 bytes array
        let mut data = big_num.to_bytes_be();
        // left padding the bytes with zeros to make sure they are 30 bytes long
        while data.len() < 30 {
            data.insert(0, 0);
        }
        // fill the data to a Num
        let mut data_arr: [u8; 30] = [0; 30];
        for i in 0..30{
            data_arr[i] = data[i];
        }
        Num{data: data_arr}
       
    }
}
#[derive(Clone, Copy)]
pub struct Data15K{
    /// a array of 512 Num 
    pub data: [Num; 512],
}

impl Data15K{
    // give a file path, read the data from the file and return a Data15K
    pub fn new(path: &str) -> Result<Data15K, String>{
        // read the data from the file
        let mut file = std::fs::File::open(path).unwrap();
        // read as bytes and read all in buffer 
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        // check the size, if the byte length > 15K, return err 
        if buffer.len() > 15*1024{
            return Err("the file is too large".to_string());
        }
        // if the byte length < 15K, padding with 0 add the edd 
        if buffer.len() == 0{
            return Err("the file is empty".to_string());
        }
        if buffer.len() < 15*1024{
            // push a indicator number which means the number of padding 0s
            buffer.push((15*1024-buffer.len() -1) as u8);
        }
        while buffer.len() < 15*1024{
            buffer.push(0);
        }
        // file the data with the buffer
        let mut data = [Num::new_zero(); 512];
        for i in 0..512{
            let mut data_arr: [u8; 30] = [0; 30];
            for j in 0..30{
                data_arr[j] = buffer[i*30+j];
            }
            data[i] = Num{data: data_arr};
        }
        Ok(Data15K{data: data})
    }

    pub fn new_rand() -> Data15K{
        let mut data = [Num::new_rand(); 512];
        for i in 0..512{
            data[i] = Num::new_rand();
        }
        Data15K{data: data}
    }
    /// export the data15K as a json file
    /// Format: ["123456789012345678901234567890", "123456789012345678901234567890", ...]
    pub fn export_json(&self, json_path: &str) -> Result<(), String>{
        // convert the data to a vector of string
        let mut data_vec: Vec<String> = Vec::new();
        for i in 0..512{
            data_vec.push(self.data[i].to_decimal());
        }
        // convert the vector to a json string
        let json = serde_json::to_string(&data_vec).unwrap();
        // write the json string to the file
        let mut file = std::fs::File::create(json_path).unwrap();
        file.write_all(json.as_bytes()).unwrap();
        Ok(())
    }
    /// export to circom json 
    pub fn export_circom_json(&self, json_path: &str) -> Result<(), String>{
        // convert the data to a vector of string
        let mut data_vec: Vec<String> = Vec::new();
        for i in 0..512{
            data_vec.push(self.data[i].to_decimal());
        }
        // build the json output format as { "in": ["123456789012345678901234567890", "123456789012345678901234567890", ...] }
        let json = format!("{{ \"in\": {} }}", serde_json::to_string(&data_vec).unwrap());
        // write the json string to the file
        let mut file = std::fs::File::create(json_path).unwrap();
        file.write_all(json.as_bytes()).unwrap();
        Ok(())
    }

    /// read json and check if the json is aligned with the data15K
    pub fn check_json(self, json_path: &str) -> bool {
        // read the json file
        let mut file = std::fs::File::open(json_path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        // convert the json string to a vector of string
        let data_vec: Vec<String> = serde_json::from_str(&contents).unwrap();
        // check if the length of the vector is 512
        if data_vec.len() != 512{
            return false;
        }
        // check if the data_vec is aligned with the data15K
        for i in 0..512{
            if data_vec[i] != self.data[i].to_decimal(){
                return false;
            }
        }
        true
    }
    /// the reverse function of new, write the data15K to a file, trim the padding 0s
    /// if trim is true, trim the padding 0s, otherwise, write all 15K bytes to the file
    pub fn to_src(&self, src_path: &str, trim: bool ) -> Result<(), String> {
        // convert the data15K to a vector of bytes
        let mut data_vec: Vec<u8> = Vec::new();
        for i in 0..512{
            for j in 0..30{
                data_vec.push(self.data[i].data[j]);
            }
        }
        if trim{
            // trim the padding 0s, until meet the indicator number, which is the number of padding 0s
            let mut i = 15*1024-1;
            let mut zero_cnt = 0; 
            while data_vec[i] == 0{
                i -= 1;
                zero_cnt += 1;
            }
            // check if the indicator number is correct
            if data_vec[i] != ((zero_cnt) as u8){
                return Err("the indicator number is not correct".to_string());
            }
            // trim the padding 0s and the indicator number
            data_vec.truncate(i);
        }
        // write the data_vec to the file
        let mut file = std::fs::File::create(src_path).unwrap();
        file.write_all(&data_vec).unwrap();

        Ok(())
    }
    /// get input json plaintext json list and convert it to a Data15K
    pub fn from_circom_pt(pt_path: &str) -> Data15K{
        let mut pt: Vec<BigUint> = Vec::new(); 
        // read the pt from pt_path
        let mut pt_file = std::fs::File::open(pt_path).unwrap();
        let mut pt_str = String::new();
        pt_file.read_to_string(&mut pt_str).unwrap();
        // parse as json value 
        let pt_json: serde_json::Value = serde_json::from_str(&pt_str).unwrap();
        // pt_json is a [string;512]
        let pt_str_vec: Vec<String> = serde_json::from_value(pt_json["in"].clone()).unwrap();
        //convert the str_vec to a biguint vec
        for i in 0..512{
            // parse the string as radix 10
            let t = BigUint::parse_bytes(pt_str_vec[i].as_bytes(), 10).unwrap();
            pt.push(t);
        }
        // convert the biguint vec to a data15k
        // first we convert each biguint to a 30 bytes array
        let mut data = [Num::new_zero(); 512];
        for i in 0..512{
            let mut data_arr: [u8; 30] = [0; 30];
            let t = pt[i].to_bytes_be();
            // the t may not be a 30 bytes long, so we need to left padding it with 0s
            for j in 0..30{
                if j < t.len(){
                    data_arr[30-t.len()+j] = t[j];
                }
            }
            data[i] = Num{data: data_arr};
        }
        // return the data15k
        Data15K{data: data}

    }
    

}

#[cfg(test)]
mod tests {
    use super::*;
    /// test the new function
    #[test]
    fn test_new_rand() {
        let data = Data15K::new_rand();
        for i in 0..512{
            println!("{}", data.data[i].to_decimal());
        }
    }
    /// test the num's demical function
    #[test]
    fn test_num_decimal(){
        let s1 = "123456789012345678901234567890";
        let num1 = Num::parse_decimal(s1);
        assert_eq!(num1.to_decimal(), s1);
    }
    /// test the Data15K's export_json function
    #[test]
    fn test_json() {
        // create a file that is 14K 
        let mut file = std::fs::File::create("test.txt").unwrap();
        let mut data = Vec::new();
        for i in 0..14*1024{
            data.push(0);
        }
        file.write_all(&data).unwrap();
        // create a data15K from the file
        let data = Data15K::new("test.txt").unwrap();
        for i in 0..512{
            println!("{}", data.data[i].to_decimal());
        }
        // export to json 
        data.export_json("test.json").unwrap();
        // check if the json is aligned with the data15K
        assert_eq!(data.check_json("test.json"), true);
        // delete all test files 
        std::fs::remove_file("test.txt").unwrap();
        std::fs::remove_file("test.json").unwrap();
    }
    /// test the Data15K's export_circom_json function
    #[test]
    fn test_circom_json() {
        // create a file that is 14K 
        let mut file = std::fs::File::create("test.txt").unwrap();
        let mut data = Vec::new();
        for i in 0..14*1024{
            data.push(0);
        }
        file.write_all(&data).unwrap();
        // create a data15K from the file
        let data = Data15K::new("test.txt").unwrap();
        for i in 0..512{
            println!("{}", data.data[i].to_decimal());
        }
        // export to circom json 
        data.export_circom_json("test_circom.json").unwrap();
        // delete it 
        std::fs::remove_file("test_circom.json").unwrap();
    }
    #[test]
    fn test_pt_2_src(){
        //read file './src.jpg' 
        let mut file = std::fs::File::open("./src.jpg").unwrap();
        let mut src1 = Vec::new();
        file.read_to_end(&mut src1).unwrap();
        // create a data15K from the file
        let data = Data15K::new("./src.jpg").unwrap();
        // export to circom json
        data.export_circom_json("test_circom.json").unwrap();
        // restore a data15K from circom json
        let data2 = Data15K::from_circom_pt("test_circom.json");
        // convert data2 to src
        data2.to_src("test_src.jpg", true).unwrap();
        // make sure the src is the same as the original file
        let mut file2 = std::fs::File::open("test_src.jpg").unwrap();
        let mut src2 = Vec::new();
        file2.read_to_end(&mut src2).unwrap();
        assert_eq!(src2.len() , src1.len()); 
        let mut dff_cnt = 0; 
        for i in 0..src2.len(){
            if src2[i] != src1[i]{
                dff_cnt += 1;
            }
        }
        // log the diff_cnt and the diff bytes in stderr 
        println!("total bytes: {}", src2.len());
        println!("diff_cnt: {}", dff_cnt);
        println!("diff bytes: ");
        for i in 0..src2.len(){
            if src2[i] != src1[i]{
                println!("{}: {} {}", i, src1[i], src2[i]);
            }
        }
        assert_eq!(dff_cnt, 0); 
        // delete the test files
        std::fs::remove_file("test_circom.json").unwrap();
        //std::fs::remove_file("test_src.jpg").unwrap();

    }
    
}