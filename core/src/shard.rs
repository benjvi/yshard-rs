use utils::error::Result;
use jq_rs;
use std::io::{self, Read};
use serde_json;
use std::fs::File;
use std::io::prelude::*;
use serde_yaml;
extern crate yaml_rust;
use yaml_rust::{YamlLoader, YamlEmitter};
extern crate sanitize_filename;

pub fn shard_yaml(outdir: &str, groupby_path: &str) -> Result<()> {
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

    eprintln!("{} yaml documents parsed in the input file", json_values.len());

    // convert list of serde_json::Value back to json string
    let json_from_yaml = serde_json::to_string(&json_values);
    match json_from_yaml {
        Ok(v) => shard_json(&v.to_string(), outdir, groupby_path),
        Err(e) => eprintln!("error: {:?}", e),
    }

    Ok(())
}

pub fn shard_json(json: &str, outdir: &str, groupby_path: &str) -> () {
    // println!("{}", json);

    // expects a json list, groups elements by the given path
    // filtering for only elements where the groupby path is present
    let jq_groupby = format!("[ .[] |  select({groupby})] | [group_by({groupby})[] | {{ (.[0] | {groupby}): . }}] | add", groupby=groupby_path);
    let group_result = jq_rs::run(&jq_groupby, json);
    match group_result {
        Ok(v) => handle_grouped_json(&v, &outdir),
        Err(e) => eprintln!("error grouping json: {:?}", e),
    }

    // get items where the groupby path is not present
    let jq_ungrouped = format!("[.[] |  select({groupby} | not)]", groupby=groupby_path);
    let ungrouped_result = jq_rs::run(&jq_ungrouped, json);
    match ungrouped_result {
        Ok(v) => handle_ungrouped_json(&v, &outdir),
        Err(e) => eprintln!("error grouping json: {:?}", e),
    }

}

fn handle_grouped_json(grouped_json: &str, outdir: &str) -> (){

    // get keys and print them
    let jq_groupby_keys = "keys";
    let result = jq_rs::run(&jq_groupby_keys, grouped_json);
    match result {
        Ok(v) => output_groups(grouped_json, &v, outdir),
        Err(e) => eprintln!("error: {:?}", e),
    }
}

fn output_groups(grouped_json: &str, groups_json: &str, outdir: &str) -> () {
    // println!("groupby keys (json): {}", groups_json);
    let groups: Vec<String> = serde_json::from_str(&groups_json).unwrap();
    // println!("groupby keys: {:#?}", groups);

    //println!("{}", grouped_json);
    for group in groups.iter() {
        let jq_group_content = format!("[.[\"{group}\"][]]", group=group);
        //println!("{}",jq_group_content);
        let result = jq_rs::run(&jq_group_content, grouped_json);
        match result {
            Ok(v) => json_to_yml_file(&v, outdir,group),
            Err(e) => eprintln!("error: {:?}", e),
        }

    }
}

fn handle_ungrouped_json(ungrouped_json: &str, outdir: &str) -> () {
    json_to_yml_file(&ungrouped_json, outdir, "__ungrouped__")
}

fn json_to_yml_file(json: &str, outdir: &str, file_name: &str) -> () {
    //TODO: skip file write for empty json

    //println!("{}", json);
    if json.len() == 0 {
        eprintln!("skipping writing empty group {} to file", file_name);
        return
    }

    let yaml_values = serde_json::from_str::<Vec<serde_yaml::Value>>(json).unwrap();

    let safe_filename =  sanitize_filename::sanitize(file_name);
    let output_filepath = format!("{0}/{1}.yml",outdir, safe_filename);

    eprintln!("writing {0} yaml docs to output file: {1}/{2}.yml", yaml_values.len(), outdir, safe_filename);

    let mut file = File::create(output_filepath).expect(&format!("Unable to create file{0}/{1}.yml",outdir, safe_filename));
    for val in yaml_values.iter() {

        //file.write_all("---".as_bytes()).expect("Unable to write yaml separator");
        let yaml_str = serde_yaml::to_string(&val).unwrap();
        file.write_all(yaml_str.as_bytes()).expect("Unable to write data");
    }

}
