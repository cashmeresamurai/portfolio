use anyhow::{Context, Result};
use polodb_core::bson::{doc, Document};
use polodb_core::Database;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct AboutMe {
    pub entry_title: String,
    pub content: String,
}

pub fn init_db() -> Result<()> {
    let db = Database::open_memory()?;
    let col = db.collection::<AboutMe>("about_me");
    let docs = vec![
        AboutMe { entry_title: "About Me".to_string(), content: "I'm 23 years old and studying Computer Science with a focus on Digital Forensics/Cyber Security in Hamburg. From a young age, I was always technically inclined, but my interest in computer science only developed during my studies. The topic that has accompanied me throughout the years is automation. I've always found it fascinating to optimize routine tasks so they can be performed entirely independently - and preferably simultaneously.".to_string() },
        AboutMe { entry_title: "Tech Stack".to_string(), content: "I mainly write code for my projects using Rust and Python. For frontend development, I use HTML with TailwindCSS, DaisyUI and HTMX to make the website more interactive".to_string() },
        AboutMe { entry_title: "Deployment and Hosting".to_string(), content: "I usually handle deployment and hosting myself by renting a Linux VPS and using Docker. This has the advantages of saving me costs, allowing me direct access to the machine if intense changes are needed, and learning from various projects - from setup to deployment.".to_string() },
        AboutMe { entry_title: "Programming Environment".to_string(), content: "My preferred programming environment is Linux. While you can program anywhere, I personally like working with terminals best. If Windows is required for development, I usually use the Developer Edition of Windows 11 in a virtual machine (which is provided directly by Microsoft).".to_string() },
    ];
    col.insert_many(docs)?;
    Ok(())
}

pub fn return_collection() -> Result<Vec<AboutMe>> {
    let db = Database::open_memory()?;
    let col = db.collection::<AboutMe>("about_me");
    let about_me_entries = col.find(None)?;
    let about_me_vec: Vec<AboutMe> = about_me_entries
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to collect about_me_entries")?;

    Ok(about_me_vec)
}
