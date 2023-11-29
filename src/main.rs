use dialoguer::{theme::ColorfulTheme, Select};
use rusqlite::{Connection, Result};
use std::process;
mod functionsql;

fn main() -> Result<()> {
    let conn: Connection = Connection::open("../fruit.db")?;

    conn.execute("create table if not exists fruits (
        id integer primary key,
        name text not null unique
    )", [])?;

    loop {
        let options: [&str; 4] = ["Cadastrar fruta", "Deletar fruta", "Ver todas as frutas", "Sair"];
        
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Escolha uma opção:")
            .default(0)
            .items(&options)
            .interact()
            .unwrap();
        
        // Use a seleção do usuário
        match selection {
            0 => functionsql::insert(&conn),
            1 => functionsql::delete(&conn),
            2 => functionsql::select_all(&conn),
            3 => {
                println!("Saindo...");
                process::exit(0);
            },
            _ => unreachable!(),
        }
    }
}

