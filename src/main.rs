use std::io::BufWriter;
use std::io::Write;
use std::path::Path;
use std::time::Duration;
use rand::prelude::SliceRandom;
use thirtyfour::{self, DesiredCapabilities, WebDriver, By};
use tokio;
use std::thread::sleep;

#[tokio::main]
async fn main() {
    let mut caps = DesiredCapabilities::chrome();
    let path = Path::new("./adblock.crx");
    let _extension_result = caps.add_extension(path).unwrap();
    let driver = WebDriver::new("http://localhost:9515", &caps)
    .await
    .unwrap();

    driver.get("https://www.mysbusiness.com/company/Pangkin-Sdn-Bhd").await.unwrap();

    let file = std::fs::OpenOptions::new().read(true).write(true).append(true).open("rawdata.txt").expect("cannot open users file");

    for i in 901749 ..= 1100000 {

        //check reg number
        let list_group_element = driver.find_element(By::ClassName("list-group")).await.unwrap();
        let list_group_item_element = list_group_element.find_element(By::ClassName("list-group-item")).await.unwrap();
        let reg_num_element = list_group_item_element.find_element(By::ClassName("col-sm-9")).await.unwrap();
        let mut reg_num = reg_num_element.text().await.unwrap();
        reg_num.pop();
        reg_num.pop();
        println!("{}", reg_num);
        if i.to_string() != reg_num {
            println!("reg number not the same, regnumber: {}, {}", reg_num, i.to_string());
            panic!()
        }

        //check company
        let h1_element = driver.find_element(By::Tag("h1")).await.unwrap();
        let company_name = h1_element.text().await.unwrap();

        //scrapping
        let mut result = String::new();
        match driver.find_element(By::ClassName("text-danger")).await {
            Ok(t) => {
                let not_active = t.inner_html().await.unwrap();
                result = not_active;
            },
            Err(_) => {
                match driver.find_element(By::ClassName("text-success")).await {
                    Ok(t) => {
                        let not_active = t.inner_html().await.unwrap();
                        result = not_active;
                    },
                    Err(_) => {
                        match driver.find_element(By::ClassName("text-warning")).await {
                            Ok(t) => {
                                let not_active = t.inner_html().await.unwrap();
                                result = not_active;
                            },
                            Err(_) => {
                                println!("cannot find text text-warning, text, text-success, text-danger class");
                                panic!();
                            },
                        }
                    },
                }
            },
        }

        let dissolved = "DISSOLVED".to_string();
        let existing = "EXISTING".to_string();
        let winding_up = "WINDING UP".to_string();

        if result != dissolved && result!= existing && result != winding_up {
            println!("result is not dissovled or existing");
            println!("result = {}", result);
            panic!()
        }



        if result == dissolved || result == winding_up {
                        //turn to another page
            let a_element = driver.find_elements(By::ClassName("panel-default")).await.unwrap();
            match a_element.len() {
                0 => {
                    println!("a_element length is 0");
                    panic!()
                },
                1 => {
                    let explore_element = a_element[0].clone();
                    let a_tag = explore_element.find_elements(By::Tag("a")).await.unwrap();
                    let button = a_tag[1].clone();
                    button.click().await.unwrap();
                },
                2 => {
                    let explore_element = a_element[1].clone();
                    let a_tag = explore_element.find_elements(By::Tag("a")).await.unwrap();
                    let button = a_tag[1].clone();
                    button.click().await.unwrap();
                }
                3 => {
                    let explore_element = a_element[2].clone();
                    let a_tag = explore_element.find_elements(By::Tag("a")).await.unwrap();
                    let button = a_tag[1].clone();
                    button.click().await.unwrap();
                },
                _ => {
                    println!("a_element length is more tha 3");
                    panic!()
                },
            }

            //sleep
            let mut rng = rand::thread_rng();
            let mut nums: Vec<i32> = (2 ..= 4).collect();
            nums.shuffle(&mut rng);
            let n = nums[0] as u64;

            sleep(Duration::from_secs(n));

            continue;
        }





        //format string
        let string = format!("{},{},{}", reg_num, company_name, result);


        //write
        println!("{}", string);
        let mut writer = BufWriter::new(&file);
        writeln!(writer, "{}", string).expect("cannot write to rawdata.txt");

        //turn to another page
        let a_element = driver.find_elements(By::ClassName("panel-default")).await.unwrap();
        match a_element.len() {
            0 => {
                println!("a_element length is 0");
                panic!()
            },
            1 => {
                let explore_element = a_element[0].clone();
                let a_tag = explore_element.find_elements(By::Tag("a")).await.unwrap();
                let button = a_tag[1].clone();
                button.click().await.unwrap();
            },
            2 => {
                let explore_element = a_element[1].clone();
                let a_tag = explore_element.find_elements(By::Tag("a")).await.unwrap();
                let button = a_tag[1].clone();
                button.click().await.unwrap();
            }
            3 => {
                let explore_element = a_element[2].clone();
                let a_tag = explore_element.find_elements(By::Tag("a")).await.unwrap();
                let button = a_tag[1].clone();
                button.click().await.unwrap();
            },
            _ => {
                println!("a_element length is more tha 3");
                panic!()
            },
        }

        //sleep
        let mut rng = rand::thread_rng();
        let mut nums: Vec<i32> = (2 ..= 4).collect();
        nums.shuffle(&mut rng);
        let n = nums[0] as u64;

        sleep(Duration::from_secs(n));

    }
 
}
