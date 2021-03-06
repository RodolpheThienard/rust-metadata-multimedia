use std::io::{stdin, Write};
use crate::scan::{scan, scan_add_tag};
use crate::scrap::scrap;
use crate::write2md::write2md;
use crate::musicfile::MusicFile;
use crate::search::search;
use crate::write2playlist::write2pls;


/// Fonction qui génère une sorti pour la recherche effectuée. 
fn output(music_files: &[MusicFile]) {
    let mut toogler = String::new();
    
    println!("Souhaitez vous l'enregistrer en json (sérialiser) ? (y / n)");
                let _ = stdin().read_line(&mut toogler);
                match toogler.as_str() {
                    "n\n" => {},
                    "y\n" => {
                        let serialized = serde_json::to_string_pretty(&music_files).unwrap();
                        let mut file = std::fs::File::create("interaction.json").unwrap();
                        file.write_all(serialized.as_bytes()).expect("Err");
                    },
                    _ => {},
                }

                println!("Souhaitez vous l'enregistrer en markdown ? (y / n)");
                toogler.clear();
                let _ = stdin().read_line(&mut toogler);
                match toogler.as_str() {
                    "n\n" => {},
                    "y\n" => {
                        write2md(music_files);
                    },
                    _ => {},
                }

                println!("Souahitez vous créer une playlist avec les musiques ? (y / n)");
                toogler.clear();
                let _ = stdin().read_line(&mut toogler);
                match toogler.as_str() {
                    "n\n" => {},
                    "y\n" => {
                        write2pls(music_files);
                    },
                    _ => {},
                }
}

/// Fonction pour aiguiller l'utilisateur
pub fn user_helper() {
    let mut buf = String::new();
    let mut toogler = String::new();
    let mut toogler2 = String::new();
    let mut path = String::new();
    let mut category = String::new();
    let mut arguments = String::new();


    println!("Scan : permet de scan un dossier pour y enregistrer 
    toutes les musiques et par la suite, modifier les informations \n");

    println!("search : permet de trier un scan ou un fichier sérialisé
    avec différents argument ( year / title / artist / album ) et differents
    modificateur ( not / and / or)\n");

    println!("scrap : permet d'ajouter / modifier une metadata ( tags ) qui sont
    album / year / title / artist. Elle récupère les informations sur internet. 
    le fichier doit avoir le pattern: artist - titre\n");

    println!("tag : permet d'ajouter / modifier une metadata ( tags ) qui sont
    album / year / title / artist\n");

    println!("Veuillez entre la commande pour continuer : ");
   
    'interact: loop {
        let _ = stdin().read_line(&mut buf);
        match buf.as_str().trim() {
            "scan" => {
                println!("où souhaitez vous scanner les musiques ?");
                stdin().read_line(&mut path).expect("Path non reconnu");
                let path = std::path::Path::new(&path[0..path.len()-1]);
                let music_files = scan(path);

                output(&music_files);

                break 'interact;},

            "search" => {
                
                println!("souhaitez vous utiliser un fichier serialise ? y/n");
                let _ = stdin().read_line(&mut toogler);

                let mut args_vec: Vec<String> = Vec::new();
                'search: loop {
                    println!("Ecrivez la categorie de la recherche que vous souhaitez faire : 
                    (Ex: year / artist / album / title");
                    let _ = stdin().read_line(&mut category);
                    println!("Ecrivez la restriction que vous souhaitez appliquer");
                    let _ = stdin().read_line(&mut arguments);
                    args_vec.push(format!{"{}={}", category[0..category.len()-1].to_string(), arguments[0..arguments.len()-1].to_string()});
                    println!("Avez vous un autre argument ? y/n");
                    let _ = stdin().read_line(&mut toogler2);
                    match toogler2.as_str().trim() {
                        "y" => {
                            println!("quel operateur souhaitez vous ajouter ? ( not / or / and)");
                            arguments.clear();
                            let _ = stdin().read_line(&mut arguments);
                            args_vec.push(arguments[0..category.len()-1].to_string());
                            arguments.clear();
                            category.clear();
                            toogler2.clear();
                        },
                        "n" => {
                            match toogler.as_str().trim() {
                                "n" => {
                                    println!("où souhaitez vous scanner les musiques ?");
                                    stdin().read_line(&mut path).expect("Path non reconnu");
                                    let path = std::path::Path::new(&path[0..path.len()-1]);
                                    let music_files = scan(path);
                                    search(&music_files, &args_vec);
                                    output(&music_files);
                                    break 'search;
                                },
                                "y" => {
                                    let deserialize: Vec<MusicFile> = serde_json::from_str(&std::fs::read_to_string("seriafile.json").expect("msg")).expect("msg");
                                    search(&deserialize, &args_vec);
                                    output(&deserialize);
                                    break 'search;
                                },
                                _ => {break 'search;},
                            }
                        },
                        _ => {break 'interact;},
                    }
                    
                }
                break 'interact;},


                "tag" => {
                    println!("où souhaitez vous modifier les tags des musiques ?");
                    stdin().read_line(&mut path).expect("Path non reconnu");
                    let path = std::path::Path::new(&path[0..path.len()-1]);

                    println!("quel categorie de tag souhaitez vous modifier ?");
                    stdin().read_line(&mut category).expect("Path non reconnu");
                    println!("quel est le nouveau tag ?");
                    stdin().read_line(&mut arguments).expect("Path non reconnu");

                    scan_add_tag(path, category.trim(), arguments.trim());
                    let music_files = scan(path);
                    output(&music_files);

                    break 'interact;
                },


                "scrap" => {
                    println!("souhaitez vous utiliser un fichier serialise ? y/n");
                    let _ = stdin().read_line(&mut toogler);

                    match toogler.as_str().trim() {
                        "n" => {
                            println!("où souhaitez vous scanner les musiques ?");
                            stdin().read_line(&mut path).expect("Path non reconnu");
                            let path = std::path::Path::new(&path[0..path.len()-1]);
                            let music_files = scan(path);
                            let _ = scrap(&music_files);
                            let output_vec: Vec<MusicFile> = scan(path);
                            output(&output_vec);
                            break 'interact;
                        },
                        "y" => {
                            let deserialize: Vec<MusicFile> = serde_json::from_str(&std::fs::read_to_string("interaction.json").expect("msg")).expect("msg");
                            let result = scrap(&deserialize);
                            let mut musics: Vec<MusicFile> = Vec::new();
                            for res in result.unwrap() {
                                musics.push(scan(std::path::Path::new(res)).pop().unwrap());
                            }
                            output(&musics);
                            break 'interact;
                        },
                        _ => {break 'interact;},
                    }
                    
                },
            _ => {},
        }
        buf.clear();
        
    }
}