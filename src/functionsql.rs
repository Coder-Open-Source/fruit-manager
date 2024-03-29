use rusqlite::{Connection, params};
use std::io;
use std::process;

pub fn select_all(conn: &Connection){
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

pub fn insert(conn: &Connection) {
    process::Command::new("clear").status().unwrap();

    println!("Qual fruta quer cadastrar no sistema?");
    let mut fruit_name: String = String::new();
    io::stdin().read_line(&mut fruit_name).unwrap();
    fruit_name = fruit_name.trim().to_uppercase();

    let query_result = conn.query_row(
        "select 1 from fruits where name = ?",
        [&fruit_name],
        |row| row.get::<_, i32>(0),
    );

    if let Ok(..) = query_result {
        println!("Está fruta ja esta cadastrada",);
    } else {
        conn.execute("insert into fruits (name) values (?)", [&fruit_name]).unwrap();
        println!("Fruta: {} cadastrada com sucesso", &fruit_name);
    };

    println!();
}

pub fn delete(conn: &Connection) {
    process::Command::new("clear").status().unwrap();

    println!("Qual fruta quer deletar do sistema?");
    let mut fruit_name: String = String::new();
    io::stdin().read_line(&mut fruit_name).unwrap();
    fruit_name = fruit_name.trim().to_uppercase();

    let query_result = conn.query_row(
        "select 1 from fruits where name = ?",
        [&fruit_name],
        |row| row.get::<_, i32>(0),
    );

    if let Ok(..) = query_result {
        conn.execute("delete from fruits where name = ?", [&fruit_name]).unwrap();
        println!("Fruta: {} deletada com sucesso", &fruit_name);
    } else {
        println!("Não existe esta fruta no sistema.",);
    }
    println!();
}
