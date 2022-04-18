/*
Author: Pyro
Purpose: to wrap around flatpak, pacman, and the AUR to provide a modern
flatpak first install script
requirements: pacman, flatpak
Package install_command format "pacman -S []"
 */
use std::process::{Command, ExitStatus, Output};
use std::{str, default, io};
use std::fs::File;
use std::io::{Write, Read};
use std::collections::HashMap;

struct PackageType{
    priority: u32,
    name: String,
    query_command: String,
    install_command: String,
    remove_command: String,
    refresh_command: String,
    update_all_command: String,
}


struct Package<'a>{
    name: String,
    packagetype: &'a PackageType,
}

impl Package<'_>{
    fn build_install_command(&self) -> String{
        let install_command_split = self.packagetype.install_command.split(" [] ");
        let install_command_vec: Vec<&str> = install_command_split.collect();
        let install_command = format!("{} {} {}", install_command_vec[0], self.name, install_command_vec[1]);
        println!("{}", install_command);
        return install_command;
    }

    fn build_search_command(&self) -> String{
        let query_command_split = self.packagetype.query_command.split(" [] ");
        let query_command_vec: Vec<&str> = query_command_split.collect();
        let query_command = format!("{} {} {}", query_command_vec[0], self.name, query_command_vec[1]);
        println!("{}", query_command);
        return query_command;
    }

    fn build_remove_command(&self) -> String{
        let remove_command_split = self.packagetype.remove_command.split(" [] ");
        let remove_command_vec: Vec<&str> = remove_command_split.collect();
        let remove_command = format!("{} {} {}", remove_command_vec[0], self.name, remove_command_vec[1]);
        println!("{}", remove_command);
        return remove_command;
    }
}


fn default_action(){
    print!("
____    __    ____  _______  __        ______   ______   .___  ___.  _______     
\\   \\  /  \\  /   / |   ____||  |      /      | /  __  \\  |   \\/   | |   ____|    
 \\   \\/    \\/   /  |  |__   |  |     |  ,----'|  |  |  | |  \\  /  | |  |__       
  \\            /   |   __|  |  |     |  |     |  |  |  | |  |\\/|  | |   __|      
   \\    /\\    /    |  |____ |  `----.|  `----.|  `--'  | |  |  |  | |  |____     
    \\__/  \\__/     |_______||_______| \\______| \\______/  |__|  |__| |_______|    
                                                                                 
.___________.  ______                                                            
|           | /  __  \\                                                           
`---|  |----`|  |  |  |                                                          
    |  |     |  |  |  |                                                          
    |  |     |  `--'  |                                                          
    |__|      \\______/                                                           
                                                                                 
 _______  __          ___   .___________..___  ___.      ___      .__   __.  __  
|   ____||  |        /   \\  |           ||   \\/   |     /   \\     |  \\ |  | |  | 
|  |__   |  |       /  ^  \\ `---|  |----`|  \\  /  |    /  ^  \\    |   \\|  | |  | 
|   __|  |  |      /  /_\\  \\    |  |     |  |\\/|  |   /  /_\\  \\   |  . `  | |  | 
|  |     |  `----./  _____  \\   |  |     |  |  |  |  /  _____  \\  |  |\\   | |__| 
|__|     |_______/__/     \\__\\  |__|     |__|  |__| /__/     \\__\\ |__| \\__| (__) 

USAGE:
flatman search $package - search by package name and install from a list

flatman install $package - install a package - will install via flatpak if availble, then pacman, then the AUR

flatman remove $package - uninstall a package 

flatman $package - same as flatman search                                                    
")
}


fn update_all_packages(packagetypes: &[PackageType; 3]){
    let mut reboot_recommended = false;
    print!("
***************************** UPDATING ALL PACKAGES PLEASE BE PATIENT **********************************
    ");
    for packagetype in packagetypes{
        let failed_string = format!("Failed to run update command for {}", packagetype.name);
        println!("**********Updating {}**********",packagetype.name);
        let output = Command::new("bash").arg("-c").arg(&packagetype.update_all_command).output().expect(&failed_string);
        let outputstring = match str::from_utf8(&output.stdout){
            Ok(v) => v,
            Err(e) => panic!("invalid utf8 {}", e)
        };
        if outputstring.contains("core/linux"){
            reboot_recommended = true;
        }
        std::io::stdout().write(&output.stdout).unwrap();
        std::io::stderr().write(&output.stderr).unwrap();
    }
    if reboot_recommended == true{
        print!("
************************************************* IMPORTANT ****************************************************************
                        A new Linux Kernel, or essential kernel drivers have been installed
                        it is highly recommended that you reboot otherwise some strange
                        hardware behavior may be observed.
****************************************************************************************************************************
        ")
    }
}


fn save_list_of_packages(packages: Vec<Package>){
    /*let package_list_file = File::create("/usr/share/flatman/package_list.txt").expect("Failed to create package list file");
    for package in packages{
        let outline = format!("{} {}", package.name, package.packagetype);
        writeln!(&mut package_list_file), outline;
    }*/
    println!("Coming soon");
} 


fn list_and_select_packages(packagelist:&Vec<Package>, command: &String){
    print!("
Name |   Repository
\n");
    let mut package_count = 0;
    let mut package_menu = HashMap::new();
    for package in packagelist{
        package_count = package_count + 1; 
        package_menu.insert(package_count.to_string(), package);
        println!("{} {}|    {}", package_count, package.name, package.packagetype.name);
    }
    println!("Selection?");
    let mut response = String::new();
    io::stdin().read_line(&mut response).expect("Failed to get user response");
    let trimmed_response = response.trim();
    for i in package_menu.keys(){
        if i == &trimmed_response{
            let package_to_install = package_menu[trimmed_response];
            let mut command_to_run = String::new();
            match command.as_str(){
                "install" => command_to_run = package_to_install.build_install_command(),
                "remove" => command_to_run = package_to_install.build_remove_command(),
                _ => println!("command not recoginzed, please use install, or remove")
            }
            println!("{}ing {} please wait", command, package_to_install.name);
            let output = Command::new("bash").arg("-c").arg(command_to_run).output().expect("Failed to run install command");
            let outputstring = match str::from_utf8(&output.stdout){
                Ok(v) => v,
                Err(e) => panic!("invalid utf8 {}", e)
            };
            print!("{}", outputstring);
            /*
            if package_to_install.packagetype.name == "flatpak".to_string(){
                let output = Command::new("bash").arg("-c").arg(install_command).output().expect("Failed to run install command");
                let outputstring = match str::from_utf8(&output.stdout){
                    Ok(v) => v,
                    Err(e) => panic!("invalid utf8 {}", e)
                };
                let outputstring_split = outputstring.split("\n");
                let output_vec: Vec<&str> = outputstring_split.collect();
                let mut flatpak_count = 0;
                for line in output_vec{
                    flatpak_count = flatpak_count + 1;
                    let package_com_format_split = line.split("/");
                    let package_com_format_vec: Vec<&str> = package_com_format_split.collect();
                    if package_com_format_vec.len() != 1{
                        let package_com_format = package_com_format_vec[1];
                        println!("{} {}", flatpak_count, package_com_format);
                    }
                }
            }*/
        }
    }
}


fn single_package_function(command: &String, name: &String, packagetypes: &[PackageType; 3]){
    let package_list = search_and_build_package(name, packagetypes);
    match command.as_str(){
        "install" => if package_list.len() > 1 {list_and_select_packages(&package_list, &command);}
        "search" => if package_list.len() > 1 {for package in package_list{println!("{} {}", package.name, package.packagetype.name)}},
        "remove" => if package_list.len() > 1 {list_and_select_packages(&package_list, &command);}
        _ => {println!("unknown Command, please read usage"); default_action();},
    }
}


fn search_and_build_package<'a>(name: &String, packagetypes: &'a [PackageType; 3])-> Vec<Package<'a>>{
    let lower_name = name.to_lowercase();
    let mut packages: Vec<Package> = Vec::new();
    for packagetype in packagetypes{
        println!("searching {} for matching packages", packagetype.name);
        let install_query_command_split = packagetype.query_command.split(" [] ");
        let install_query_vec: Vec<&str> = install_query_command_split.collect();
        let install_query_name = format!("{} {}", install_query_vec[0], &name);
        let output = Command::new("bash").arg("-c").arg(install_query_name).output().expect("Failed to run query command");
        if ExitStatus::success(&output.status){
            let output_success = str::from_utf8(&output.stdout).expect("error getting output");
            if output_success.contains("No matches found"){
                println!("no flatpaks found");
            }
            else{
                if packagetype.name == "flatpak".to_string(){
                    let output_success_split = output_success.split("\n");
                    let mut output_success_vec: Vec<&str> = output_success_split.collect();
                    output_success_vec.pop();
                    for out_name in output_success_vec{
                        if out_name.to_lowercase().contains(&lower_name){
                            let out_name_vec: Vec<&str> = out_name.split("\t").collect();
                            let format_out_name = out_name_vec[1];
                            let new_package = Package{name: format_out_name.to_string(), packagetype: packagetype };
                            packages.push(new_package);
                        }
                    }
                }
                else if packagetype.name == "pacman".to_string() {
                    let output_success_split = output_success.split("\n");
                    let output_success_vec: Vec<&str> = output_success_split.collect();
                    for line in output_success_vec{
                        let line = line.split(" ");
                        let line: Vec<&str> = line.collect();
                        if line[0].len() != 0{
                            let package = line[0].split("/");
                            let package: Vec<&str> = package.collect();
                            let package = package[1].trim();
                            if package.to_lowercase().contains(&lower_name){
                                let new_package = Package{name: package.to_string(), packagetype: packagetype};
                                packages.push(new_package);
                            }
                        }
                    }
                }
            }
        }
    }
    return packages;
}


fn main() {
    let pacman = PackageType{priority: 2,
        name: "pacman".to_string(),
        query_command: "pacman -Ss [] ".to_string(),
        install_command: "sudo pacman -S [] ".to_string(),
        remove_command: "sudo pacman -Rs [] ".to_string(),
        refresh_command: "sudo pacman -Sy".to_string(),
        update_all_command: "sudo pacman -Syu --noconfirm".to_string(),
    };

    let flatpak = PackageType{
        priority: 1,
        name: "flatpak".to_string(),
        query_command: "flatpak search --columns=name,application id  [] ".to_string(),
        install_command: "flatpak install --noninteractive [] ".to_string(),
        remove_command: "flatpak remove --noninteractive [] ".to_string(),
        refresh_command: "echo 'non_needed'".to_string(),
        update_all_command: "flatpak update -y".to_string(),
    };
    
    let aur = PackageType{
        priority: 3,
        name: "AUR".to_string(),
        query_command: "to don [] ".to_string(),
        install_command: "to do [] ".to_string(),
        remove_command: "to do [] ".to_string(),
        refresh_command: "to do".to_string(),
        update_all_command: "echo 'todo'".to_string(),
    };
    let packagetypes: [PackageType; 3] = [pacman, flatpak, aur];
    let args: Vec<String> = std::env::args().collect();
    match args.len(){
        1 => default_action(),
        2 => match args[1].as_str(){
            "update" => update_all_packages(&packagetypes),
            _ => default_action()
        }
        3 => single_package_function(&args[1], &args[2], &packagetypes),
        _ => default_action()
    }
}
