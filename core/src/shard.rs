use utils::error::Result;
use jq_rs;
use std::io::{self, Read};
use serde_json;
use std::fs::File;
use std::io::prelude::*;
use serde_yaml;
extern crate yaml_rust;
use yaml_rust::{YamlLoader, YamlEmitter};

pub fn shard_yaml() -> Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let docs = YamlLoader::load_from_str(&buffer).unwrap();

    // need to convert multiple yaml documents to a list containing json objects
    // serde_yaml can only load a single yaml document so doesn't help us
    // if we create a vector of json::Value then dump it to string it works

    let json_values: Vec<_> = docs.iter().map( |doc| {
        let mut out_str = String::new();
        let mut emitter = YamlEmitter::new(&mut out_str);
        emitter.dump(doc).unwrap();
        serde_yaml::from_str::<serde_json::Value>(&out_str).unwrap()
    }).collect();

    // convert list of serde_json::Value back to json string
    let json_from_yaml = serde_json::to_string(&json_values);
    match json_from_yaml {
        Ok(v) => shard_json(&v.to_string()),
        Err(e) => println!("error: {:?}", e),
    }

    Ok(())
}

pub fn shard_json(json: &str) -> () {
    println!("{}", json);

    let outdir = "e2e-results/simple";
    let groupby_path = "kind";

    // expects a json list, groups elements by the given path
    // filtering for only elements where the groupby path is present
    let jq_groupby = format!("[ .[] |  select(.{groupby})] | [group_by(.{groupby})[] | {{ (.[0] | .{groupby}): . }}] | add", groupby=groupby_path);
    let group_result = jq_rs::run(&jq_groupby, json);
    match group_result {
        Ok(v) => handle_grouped_json(&v, &outdir),
        Err(e) => println!("error grouping json: {:?}", e),
    }

    // get items where the groupby path is not present
    let jq_ungrouped = format!(".[] |  select(.{groupby} | not)", groupby=groupby_path);
    let ungrouped_result = jq_rs::run(&jq_ungrouped, json);
    match ungrouped_result {
        Ok(v) => handle_ungrouped_json(&v, &outdir),
        Err(e) => println!("error grouping json: {:?}", e),
    }

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
