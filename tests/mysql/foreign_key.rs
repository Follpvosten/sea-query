use super::*;

#[test]
fn create_1() {
    assert_eq!(
        ForeignKey::create()
            .name("FK_2e303c3a712662f1fc2a4d0aad6")
            .table(Char::Table, Font::Table)
            .col(Char::FontId, Font::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_string(MysqlQueryBuilder),
        vec![
            "ALTER TABLE `character`",
            "ADD CONSTRAINT `FK_2e303c3a712662f1fc2a4d0aad6`",
            "FOREIGN KEY `FK_2e303c3a712662f1fc2a4d0aad6` (`font_id`) REFERENCES `font` (`id`)",
            "ON DELETE CASCADE ON UPDATE CASCADE",
        ].join(" ")
    );
}

#[test]
fn drop_1() {
    assert_eq!(
        ForeignKey::drop()
            .name("FK_2e303c3a712662f1fc2a4d0aad6")
            .table(Char::Table)
            .to_string(MysqlQueryBuilder),
        "ALTER TABLE `character` DROP FOREIGN KEY `FK_2e303c3a712662f1fc2a4d0aad6`"
    );
}