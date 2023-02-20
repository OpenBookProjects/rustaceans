use std::collections::BTreeMap;

pub fn tally(match_results: &str) -> String {
    /* unimplemented!(
        "Given the result of the played matches '{match_results}' return a properly formatted tally table string."
    ); */
    let mut record = BTreeMap::<&str,BTreeMap<&str,i32>>::new();
    let lines = match_results.split_terminator('\n').collect::<Vec<&str>>();
    for line in lines{
        let items = line.split_terminator(';').collect::<Vec<&str>>();
        for item in &items[0..2]{
            if !record.contains_key(item){
                record.insert(item,BTreeMap::<&str,i32>::new());
                record.get_mut(item).unwrap().insert("MP",0);
                record.get_mut(item).unwrap().insert("W",0);
                record.get_mut(item).unwrap().insert("D",0);
                record.get_mut(item).unwrap().insert("L",0);
                record.get_mut(item).unwrap().insert("P",0);
            }
            *record.get_mut(item).unwrap().get_mut("MP").unwrap() +=1;
        }
        match items[2]{
            "win" =>{
                *record.get_mut(items[0]).unwrap().get_mut("W").unwrap() +=1;
                *record.get_mut(items[0]).unwrap().get_mut("P").unwrap() +=3;
                *record.get_mut(items[1]).unwrap().get_mut("L").unwrap() +=1;
            },
            "draw" =>{
                *record.get_mut(items[0]).unwrap().get_mut("D").unwrap() +=1;
                *record.get_mut(items[0]).unwrap().get_mut("P").unwrap() +=1;
                *record.get_mut(items[1]).unwrap().get_mut("D").unwrap() +=1;
                *record.get_mut(items[1]).unwrap().get_mut("P").unwrap() +=1;
            },
            "loss" =>{
                *record.get_mut(items[0]).unwrap().get_mut("L").unwrap() +=1;
                *record.get_mut(items[1]).unwrap().get_mut("W").unwrap() +=1;
                *record.get_mut(items[1]).unwrap().get_mut("P").unwrap() +=3;
            },
            _=>panic!()
        }
    }
    let mut bb_record = BTreeMap::<&i32,Vec<(&str,&BTreeMap<&str,i32>)>>::new();
    for (team,record) in record.iter(){
        if !bb_record.contains_key(record.get("P").unwrap()){
            bb_record.insert(record.get("P").unwrap(), Vec::<(&str,&BTreeMap<&str,i32>)>::new());
        }
        bb_record.get_mut(record.get("P").unwrap()).unwrap().push((team,record));
    }
    let mut table = "Team                           | MP |  W |  D |  L |  P\n".to_string();
    for (_,record) in bb_record.iter().rev(){
        for item in record {
            // format! very hard, need copy from test cases...
            table +=format!("{:<31}| {:>2} | {:>2} | {:>2} | {:>2} | {:>2}\n", 
                            item.0,
                            item.1.get("MP").unwrap(),
                            item.1.get("W").unwrap(),
                            item.1.get("D").unwrap(),
                            item.1.get("L").unwrap(),
                            item.1.get("P").unwrap(),
                            ).as_str();
        }
    }
    table.strip_suffix('\n').unwrap().to_string()

}

