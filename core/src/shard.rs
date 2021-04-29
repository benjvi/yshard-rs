use utils::error::Result;
use jq_rs;
use std::io::{self, Read};
use serde_json::{self, json};
use std::fs::File;
use std::io::prelude::*;

pub fn shard() -> Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let outdir = "e2e-results/simple";
    let groupby_path = "kind";

    // expects a json list, groups elements by the given path
    // filtering for only elements where the groupby path is present
    let jq_groupby = format!("[ .[] |  select(.{groupby})] | [group_by(.{groupby})[] | {{ (.[0] | .{groupby}): . }}] | add", groupby=groupby_path);
    let group_result = jq_rs::run(&jq_groupby, &buffer);
    match group_result {
        Ok(v) => handle_grouped_json(&v, &outdir),
        Err(e) => println!("error grouping json: {:?}", e),
    }

    // get items where the groupby path is not present
    let jq_ungrouped = format!(".[] |  select(.{groupby} | not)", groupby=groupby_path);
    let ungrouped_result = jq_rs::run(&jq_ungrouped, &buffer);
    match ungrouped_result {
        Ok(v) => handle_ungrouped_json(&v, &outdir),
        Err(e) => println!("error grouping json: {:?}", e),
    }

    Ok(())
}

fn handle_grouped_json(grouped_json: &str, outdir: &str) -> (){
    println!("grouped input successfully");

    // get keys and print them
    let jq_groupby_keys = "keys";
    let result = jq_rs::run(&jq_groupby_keys, grouped_json);
    match result {
        Ok(v) => output_groups(grouped_json, &v, outdir),
        Err(e) => println!("error: {:?}", e),
    }
}

fn output_groups(grouped_json: &str, groups_json: &str, outdir: &str) -> () {
    println!("groupby keys (json): {}", groups_json);
    let groups: Vec<String> = serde_json::from_str(&groups_json).unwrap();
    println!("groupby keys: {:#?}", groups);

    println!("{}", grouped_json);
    for group in groups.iter() {
        let jq_group_content = format!("[.[\"{group}\"][]]", group=group);
        println!("{}",jq_group_content);
        let result = jq_rs::run(&jq_group_content, grouped_json);
        match result {
            Ok(v) => json_to_yml_file(&v, outdir,group),
            Err(e) => println!("error: {:?}", e),
        }

    }
}

fn handle_ungrouped_json(ungrouped_json: &str, outdir: &str) -> () {
    println!("handled ungrouped input successfully");
    json_to_yml_file(&ungrouped_json, outdir, "__ungrouped__")
}

fn json_to_yml_file(json: &str, outdir: &str, file_name: &str) -> () {
    //TODO: skip file write for empty json
    println!("Writing json to output file: {0}/{1}.json", outdir, file_name);
    println!("{}", json);
    let output_filepath = format!("{0}/{1}.json",outdir, file_name);
    let mut file = File::create(output_filepath).expect("Unable to create file");
    file.write_all(json.as_bytes()).expect("Unable to write data");
}
