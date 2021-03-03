use postgres::{Client, NoTls, Row};
use sea_query::{ColumnDef, Expr, Func, Iden, Order, PostgresQueryBuilder, Query, Table, Values};

fn main() {
    let mut client = Client::connect("postgresql://query:query@localhost/query_test", NoTls).unwrap();

    // Schema

    let sql = Table::create()
        .table(Character::Table)
        .create_if_not_exists()
        .col(ColumnDef::new(Character::Id).big_integer().not_null().primary_key())
        .col(ColumnDef::new(Character::FontSize).big_integer())
        .col(ColumnDef::new(Character::Character).string())
        .col(ColumnDef::new(Character::JsonField).json_binary())
        .build(PostgresQueryBuilder);

    let result = client.batch_execute(&sql).unwrap();
    println!("Create table character: {:?}\n", result);

    // Create
    let item = CharacterStruct {
        id: 1,
        character: "a".into(),
        font_size: 12,
        json_field: serde_json::json! {{
            "a": 25.0,
            "b": "whatever",
            "c": {
                "another": "object",
                "bla": 1
            }
        }},
    };
    let (sql, values) = Query::insert()
        .into_table(Character::Table)
        .columns(vec![
            Character::Id, Character::Character, Character::FontSize, Character::JsonField,
        ])
        .json(serde_json::to_value(item).unwrap())
        .build(PostgresQueryBuilder);

    let result = client.execute(sql.as_str(), &Values::from(values).as_params());
    println!("Insert into character: {:?}\n", result);

    // Read

    let (sql, values) = Query::select()
        .columns(vec![
            Character::Id, Character::Character, Character::FontSize, Character::JsonField,
        ])
        .from(Character::Table)
        .order_by(Character::Id, Order::Desc)
        .limit(1)
        .build(PostgresQueryBuilder);

    let rows = client.query(sql.as_str(), &Values::from(values).as_params()).unwrap();
    println!("Select one from character:");
    let mut id = None;
    for row in rows.into_iter() {
        let item = CharacterStruct::from(row);
        println!("{:?}", item);
        id = Some(item.id);
    }
    let id = id.unwrap();
    println!();

    // Update

    let (sql, values) = Query::update()
        .table(Character::Table)
        .values(vec![
            (Character::FontSize, 24i64.into()),
        ])
        .and_where(Expr::col(Character::Id).eq(id))
        .build(PostgresQueryBuilder);

    let result = client.execute(sql.as_str(), &Values::from(values).as_params());
    println!("Update character: {:?}\n", result);

    // Read

    let (sql, values) = Query::select()
        .columns(vec![
            Character::Id, Character::Character, Character::FontSize, Character::JsonField,
        ])
        .from(Character::Table)
        .order_by(Character::Id, Order::Desc)
        .limit(1)
        .build(PostgresQueryBuilder);

    let rows = client.query(sql.as_str(), &Values::from(values).as_params()).unwrap();
    println!("Select one from character:");
    for row in rows.into_iter() {
        let item = CharacterStruct::from(row);
        println!("{:?}", item);
    }
    println!();

    // Delete

    let (sql, values) = Query::delete()
        .from_table(Character::Table)
        .and_where(Expr::col(Character::Id).eq(id))
        .build(PostgresQueryBuilder);

    let result = client.execute(sql.as_str(), &Values::from(values).as_params());
    println!("Delete character: {:?}\n", result);

    // Count

    let (sql, values) = Query::select()
        .from(Character::Table)
        .expr(Func::count(Expr::col(Character::Id)))
        .build(PostgresQueryBuilder);

    let row = client.query_one(sql.as_str(), &Values::from(values).as_params()).unwrap();
    print!("Count character: ");
    let count: i64 = row.try_get(0).unwrap();
    println!("{}", count);
}

#[derive(Iden)]
enum Character {
    Table,
    Id,
    Character,
    FontSize,
    JsonField,
}

#[derive(Debug, serde::Serialize)]
struct CharacterStruct {
    id: i64,
    character: String,
    font_size: i64,
    json_field: serde_json::Value,
}

impl From<Row> for CharacterStruct {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            character: row.get("character"),
            font_size: row.get("font_size"),
            json_field: row.get("json_field"),
        }
    }
}
