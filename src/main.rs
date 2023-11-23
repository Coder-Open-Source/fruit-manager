use dialoguer::{theme::ColorfulTheme, Select};
use rusqlite::{Connection, Result, params};
use std::io;
use std::process;

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
            0 => insert(&conn),
            1 => delete(&conn),
            2 => select_all(&conn),
            3 => {
                println!("Saindo...");
                process::exit(0)
            },
            _ => unreachable!(),
        }
    }
}

fn select_all(conn: &Connection){
    process::Command::new("clear").status().unwrap();

    let mut stmt = conn.prepare("SELECT * FROM fruits").unwrap();
    let fruit_iter = stmt.query_map(params![], |row| {
        let value: String = row.get(1)?;
        Ok(value)
    }).unwrap();

    println!("Frutas cadastradas:");
    for fruit in fruit_iter {
        println!("{}",fruit.unwrap().trim());
    }
    println!();
}

fn insert(conn: &Connection) {
    process::Command::new("clear").status().unwrap();

    println!("Qual fruta quer cadastrar no sistema?");
    let mut fruit_name: String = String::new();
    let _ = io::stdin().read_line(&mut fruit_name);
    fruit_name = fruit_name.trim().to_uppercase();

    let query_result = conn.query_row(
        "select 1 from fruits where name = ?",
        [&fruit_name],
        |row| row.get::<_, i32>(0),
    );

    if let Ok(..) = query_result {
        println!("Está fruta ja esta cadastrada",);
    } else {
        let _ = conn.execute("insert into fruits (name) values (?)", [&fruit_name]);
        println!("Fruta: {} cadastrada com sucesso", &fruit_name);
    };

    println!();
}

fn delete(conn: &Connection) {
    process::Command::new("clear").status().unwrap();

    println!("Qual fruta quer deletar do sistema?");
    let mut fruit_name: String = String::new();
    let _ = io::stdin().read_line(&mut fruit_name);
    fruit_name = fruit_name.trim().to_uppercase();

    let query_result = conn.query_row(
        "select 1 from fruits where name = ?",
        [&fruit_name],
        |row| row.get::<_, i32>(0),
    );

    if let Ok(..) = query_result {
        let _ = conn.execute("delete from fruits where name = ?", [&fruit_name]);
        println!("Fruta: {} deletada com sucesso", &fruit_name);
    } else {
        println!("Não existe esta fruta no sistema.",);
    }
    println!();
}
