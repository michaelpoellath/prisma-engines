use database_introspection::*;
use datamodel::{
    common::PrismaType, Datamodel, Field, FieldArity, FieldType, IdInfo, IdStrategy, Model, OnDeleteStrategy,
    RelationInfo,
};
use introspection_command::calculate_model;
use log::LevelFilter;
use pretty_assertions::assert_eq;
use std::sync::atomic::{AtomicBool, Ordering};

static IS_SETUP: AtomicBool = AtomicBool::new(false);

fn setup() {
    let is_setup = IS_SETUP.load(Ordering::Relaxed);
    if is_setup {
        return;
    }

    let log_level = match std::env::var("TEST_LOG")
        .unwrap_or("warn".to_string())
        .to_lowercase()
        .as_ref()
    {
        "trace" => LevelFilter::Trace,
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        _ => LevelFilter::Warn,
    };
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!("[{}][{}] {}", record.target(), record.level(), message))
        })
        .level(log_level)
        .chain(std::io::stdout())
        .apply()
        .expect("fern configuration");

    IS_SETUP.store(true, Ordering::Relaxed);
}

#[test]
fn a_data_model_can_be_generated_from_a_schema() {
    setup();

    let col_types = vec![
        ColumnTypeFamily::Int,
        ColumnTypeFamily::Float,
        ColumnTypeFamily::Boolean,
        ColumnTypeFamily::String,
        ColumnTypeFamily::DateTime,
        ColumnTypeFamily::Binary,
        ColumnTypeFamily::Json,
        ColumnTypeFamily::Uuid,
        ColumnTypeFamily::Geometric,
        ColumnTypeFamily::LogSequenceNumber,
        ColumnTypeFamily::TextSearch,
        ColumnTypeFamily::TransactionId,
    ];

    let ref_data_model = Datamodel {
        models: vec![Model {
            database_name: None,
            name: "Table1".to_string(),
            documentation: None,
            is_embedded: false,
            is_generated: false,
            fields: col_types
                .iter()
                .map(|col_type| {
                    let field_type = match col_type {
                        ColumnTypeFamily::Boolean => FieldType::Base(PrismaType::Boolean),
                        ColumnTypeFamily::DateTime => FieldType::Base(PrismaType::DateTime),
                        ColumnTypeFamily::Float => FieldType::Base(PrismaType::Float),
                        ColumnTypeFamily::Int => FieldType::Base(PrismaType::Int),
                        ColumnTypeFamily::String => FieldType::Base(PrismaType::String),
                        // XXX: We made a conscious decision to punt on mapping of ColumnTypeFamily
                        // variants that don't yet have corresponding PrismaType variants
                        _ => FieldType::Base(PrismaType::String),
                    };
                    Field {
                        name: col_type.to_string(),
                        arity: FieldArity::Optional,
                        field_type,
                        database_name: None,
                        default_value: None,
                        is_unique: false,
                        id_info: None,
                        scalar_list_strategy: None,
                        documentation: None,
                        is_generated: false,
                        is_updated_at: false,
                    }
                })
                .collect(),
        }],
        enums: vec![],
    };

    let schema = DatabaseSchema {
        tables: vec![Table {
            name: "Table1".to_string(),
            columns: col_types
                .iter()
                .map(|family| Column {
                    name: family.to_string(),
                    tpe: ColumnType {
                        raw: "raw".to_string(),
                        family: family.to_owned(),
                    },
                    arity: ColumnArity::Nullable,
                    default: None,
                    auto_increment: false,
                })
                .collect(),
            indices: vec![],
            primary_key: Some(PrimaryKey {
                columns: vec!["primary_col".to_string()],
            }),
            foreign_keys: vec![],
        }],
        enums: vec![],
        sequences: vec![],
    };
    let data_model = calculate_model(&schema).expect("calculate data model");

    assert_eq!(data_model, ref_data_model);
}

#[test]
fn arity_is_preserved_when_generating_data_model_from_a_schema() {
    setup();

    let ref_data_model = Datamodel {
        models: vec![Model {
            database_name: None,
            name: "Table1".to_string(),
            documentation: None,
            is_embedded: false,
            fields: vec![
                Field {
                    name: "optional".to_string(),
                    arity: FieldArity::Optional,
                    field_type: FieldType::Base(PrismaType::Int),
                    database_name: None,
                    default_value: None,
                    is_unique: false,
                    id_info: None,
                    scalar_list_strategy: None,
                    documentation: None,
                    is_generated: false,
                    is_updated_at: false,
                },
                Field {
                    name: "required".to_string(),
                    arity: FieldArity::Required,
                    field_type: FieldType::Base(PrismaType::Int),
                    database_name: None,
                    default_value: None,
                    is_unique: false,
                    id_info: None,
                    scalar_list_strategy: None,
                    documentation: None,
                    is_generated: false,
                    is_updated_at: false,
                },
                Field {
                    name: "list".to_string(),
                    arity: FieldArity::List,
                    field_type: FieldType::Base(PrismaType::Int),
                    database_name: None,
                    default_value: None,
                    is_unique: false,
                    id_info: None,
                    scalar_list_strategy: None,
                    documentation: None,
                    is_generated: false,
                    is_updated_at: false,
                },
            ],
            is_generated: false,
        }],
        enums: vec![],
    };

    let schema = DatabaseSchema {
        tables: vec![Table {
            name: "Table1".to_string(),
            columns: vec![
                Column {
                    name: "optional".to_string(),
                    tpe: ColumnType {
                        raw: "raw".to_string(),
                        family: ColumnTypeFamily::Int,
                    },
                    arity: ColumnArity::Nullable,
                    default: None,
                    auto_increment: false,
                },
                Column {
                    name: "required".to_string(),
                    tpe: ColumnType {
                        raw: "raw".to_string(),
                        family: ColumnTypeFamily::Int,
                    },
                    arity: ColumnArity::Required,
                    default: None,
                    auto_increment: false,
                },
                Column {
                    name: "list".to_string(),
                    tpe: ColumnType {
                        raw: "raw".to_string(),
                        family: ColumnTypeFamily::Int,
                    },
                    arity: ColumnArity::List,
                    default: None,
                    auto_increment: false,
                },
            ],
            indices: vec![],
            primary_key: Some(PrimaryKey {
                columns: vec!["primary_col".to_string()],
            }),
            foreign_keys: vec![],
        }],
        enums: vec![],
        sequences: vec![],
    };
    let data_model = calculate_model(&schema).expect("calculate data model");

    assert_eq!(data_model, ref_data_model);
}

#[test]
fn primary_key_is_preserved_when_generating_data_model_from_a_schema() {
    setup();

    let ref_data_model = Datamodel {
        models: vec![
            // Model with auto-incrementing primary key
            Model {
                database_name: None,
                name: "Table1".to_string(),
                documentation: None,
                is_embedded: false,
                fields: vec![Field {
                    name: "primary".to_string(),
                    arity: FieldArity::Required,
                    field_type: FieldType::Base(PrismaType::Int),
                    database_name: None,
                    default_value: None,
                    is_unique: false,
                    id_info: Some(IdInfo {
                        strategy: IdStrategy::Auto,
                        sequence: None,
                    }),
                    scalar_list_strategy: None,
                    documentation: None,
                    is_generated: false,
                    is_updated_at: false,
                }],
                is_generated: false,
            },
            // Model with non-auto-incrementing primary key
            Model {
                database_name: None,
                name: "Table2".to_string(),
                documentation: None,
                is_embedded: false,
                fields: vec![Field {
                    name: "primary".to_string(),
                    arity: FieldArity::Required,
                    field_type: FieldType::Base(PrismaType::Int),
                    database_name: None,
                    default_value: None,
                    is_unique: false,
                    id_info: Some(IdInfo {
                        strategy: IdStrategy::None,
                        sequence: None,
                    }),
                    scalar_list_strategy: None,
                    documentation: None,
                    is_generated: false,
                    is_updated_at: false,
                }],
                is_generated: false,
            },
        ],
        enums: vec![],
    };

    let schema = DatabaseSchema {
        tables: vec![
            Table {
                name: "Table1".to_string(),
                columns: vec![Column {
                    name: "primary".to_string(),
                    tpe: ColumnType {
                        raw: "integer".to_string(),
                        family: ColumnTypeFamily::Int,
                    },
                    arity: ColumnArity::Required,
                    default: None,
                    auto_increment: true,
                }],
                indices: vec![],
                primary_key: Some(PrimaryKey {
                    columns: vec!["primary".to_string()],
                }),
                foreign_keys: vec![],
            },
            Table {
                name: "Table2".to_string(),
                columns: vec![Column {
                    name: "primary".to_string(),
                    tpe: ColumnType {
                        raw: "integer".to_string(),
                        family: ColumnTypeFamily::Int,
                    },
                    arity: ColumnArity::Required,
                    default: None,
                    auto_increment: false,
                }],
                indices: vec![],
                primary_key: Some(PrimaryKey {
                    columns: vec!["primary".to_string()],
                }),
                foreign_keys: vec![],
            },
        ],
        enums: vec![],
        sequences: vec![],
    };
    let data_model = calculate_model(&schema).expect("calculate data model");

    assert_eq!(data_model, ref_data_model);
}

#[test]
fn primary_key_is_preserved_when_generating_data_model_from_a_schema() {
    setup();

    let ref_data_model = Datamodel {
        models: vec![
            // Model with auto-incrementing primary key
            Model {
                database_name: None,
                name: "Table1".to_string(),
                documentation: None,
                is_embedded: false,
                fields: vec![Field {
                    name: "primary".to_string(),
                    arity: FieldArity::Required,
                    field_type: FieldType::Base(PrismaType::Int),
                    database_name: None,
                    default_value: None,
                    is_unique: true,
                    id_info: Some(IdInfo {
                        strategy: IdStrategy::Auto,
                        sequence: None,
                    }),
                    scalar_list_strategy: None,
                    documentation: None,
                    is_generated: false,
                    is_updated_at: false,
                }],
                is_generated: false,
            },
            // Model with non-auto-incrementing primary key
            Model {
                database_name: None,
                name: "Table2".to_string(),
                documentation: None,
                is_embedded: false,
                fields: vec![Field {
                    name: "primary".to_string(),
                    arity: FieldArity::Required,
                    field_type: FieldType::Base(PrismaType::Int),
                    database_name: None,
                    default_value: None,
                    is_unique: true,
                    id_info: Some(IdInfo {
                        strategy: IdStrategy::None,
                        sequence: None,
                    }),
                    scalar_list_strategy: None,
                    documentation: None,
                    is_generated: false,
                    is_updated_at: false,
                }],
                is_generated: false,
            },
        ],
        enums: vec![],
    };

    let schema = DatabaseSchema {
        tables: vec![
            Table {
                name: "Table1".to_string(),
                columns: vec![Column {
                    name: "primary".to_string(),
                    tpe: ColumnType {
                        raw: "integer".to_string(),
                        family: ColumnTypeFamily::Int,
                    },
                    arity: ColumnArity::Required,
                    default: None,
                    auto_increment: true,
                    is_unique: true,
                }],
                indices: vec![],
                primary_key: Some(PrimaryKey {
                    columns: vec!["primary".to_string()],
                }),
                foreign_keys: vec![],
            },
            Table {
                name: "Table2".to_string(),
                columns: vec![Column {
                    name: "primary".to_string(),
                    tpe: ColumnType {
                        raw: "integer".to_string(),
                        family: ColumnTypeFamily::Int,
                    },
                    arity: ColumnArity::Required,
                    default: None,
                    auto_increment: false,
                    is_unique: true,
                }],
                indices: vec![],
                primary_key: Some(PrimaryKey {
                    columns: vec!["primary".to_string()],
                }),
                foreign_keys: vec![],
            },
        ],
        enums: vec![],
        sequences: vec![],
    };
    let data_model = calculate_model(&schema).expect("calculate data model");

    assert_eq!(data_model, ref_data_model);
}

#[test]
fn uniqueness_is_preserved_when_generating_data_model_from_a_schema() {
    setup();

    let ref_data_model = Datamodel {
        models: vec![Model {
            database_name: None,
            name: "Table1".to_string(),
            documentation: None,
            is_embedded: false,
            fields: vec![
                Field {
                    name: "non-unique".to_string(),
                    arity: FieldArity::Optional,
                    field_type: FieldType::Base(PrismaType::Int),
                    database_name: None,
                    default_value: None,
                    is_unique: false,
                    id_info: None,
                    scalar_list_strategy: None,
                    documentation: None,
                    is_generated: false,
                    is_updated_at: false,
                },
                Field {
                    name: "unique".to_string(),
                    arity: FieldArity::Required,
                    field_type: FieldType::Base(PrismaType::Int),
                    database_name: None,
                    default_value: None,
                    is_unique: true,
                    id_info: None,
                    scalar_list_strategy: None,
                    documentation: None,
                    is_generated: false,
                    is_updated_at: false,
                },
            ],
            is_generated: false,
        }],
        enums: vec![],
    };

    let schema = DatabaseSchema {
        tables: vec![Table {
            name: "Table1".to_string(),
            columns: vec![
                Column {
                    name: "non-unique".to_string(),
                    tpe: ColumnType {
                        raw: "raw".to_string(),
                        family: ColumnTypeFamily::Int,
                    },
                    arity: ColumnArity::Nullable,
                    default: None,
                    auto_increment: false,
                },
                Column {
                    name: "unique".to_string(),
                    tpe: ColumnType {
                        raw: "raw".to_string(),
                        family: ColumnTypeFamily::Int,
                    },
                    arity: ColumnArity::Required,
                    default: None,
                    auto_increment: false,
                },
            ],
            indices: vec![Index {
                name: "unique".to_string(),
                columns: vec!["unique".to_string()],
                tpe: IndexType::Unique,
            }],
            primary_key: None,
            foreign_keys: vec![],
        }],
        enums: vec![],
        sequences: vec![],
    };
    let data_model = calculate_model(&schema).expect("calculate data model");

    assert_eq!(data_model, ref_data_model);
}

#[test]
fn foreign_keys_are_preserved_when_generating_data_model_from_a_schema() {
    setup();

    let ref_data_model = Datamodel {
        models: vec![
            Model {
                database_name: None,
                name: "City".to_string(),
                documentation: None,
                is_embedded: false,
                fields: vec![
                    Field {
                        name: "id".to_string(),
                        arity: FieldArity::Required,
                        field_type: FieldType::Base(PrismaType::Int),
                        database_name: None,
                        default_value: None,
                        is_unique: false,
                        id_info: Some(IdInfo {
                            strategy: IdStrategy::Auto,
                            sequence: None,
                        }),
                        scalar_list_strategy: None,
                        documentation: None,
                        is_generated: false,
                        is_updated_at: false,
                    },
                    Field {
                        name: "name".to_string(),
                        arity: FieldArity::Required,
                        field_type: FieldType::Base(PrismaType::String),
                        database_name: None,
                        default_value: None,
                        is_unique: false,
                        id_info: None,
                        scalar_list_strategy: None,
                        documentation: None,
                        is_generated: false,
                        is_updated_at: false,
                    },
                ],
                is_generated: false,
            },
            Model {
                database_name: None,
                name: "User".to_string(),
                documentation: None,
                is_embedded: false,
                fields: vec![
                    Field {
                        name: "id".to_string(),
                        arity: FieldArity::Required,
                        field_type: FieldType::Base(PrismaType::Int),
                        database_name: None,
                        default_value: None,
                        is_unique: false,
                        id_info: None,
                        scalar_list_strategy: None,
                        documentation: None,
                        is_generated: false,
                        is_updated_at: false,
                    },
                    Field {
                        name: "city-id".to_string(),
                        arity: FieldArity::Required,
                        field_type: FieldType::Relation(RelationInfo {
                            name: "".to_string(),
                            to: "City".to_string(),
                            to_fields: vec!["id".to_string()],
                            on_delete: OnDeleteStrategy::None,
                        }),
                        database_name: None,
                        default_value: None,
                        is_unique: false,
                        id_info: None,
                        scalar_list_strategy: None,
                        documentation: None,
                        is_generated: false,
                        is_updated_at: false,
                    },
                    Field {
                        name: "city-name".to_string(),
                        arity: FieldArity::Required,
                        field_type: FieldType::Relation(RelationInfo {
                            name: "".to_string(),
                            to: "City".to_string(),
                            to_fields: vec!["name".to_string()],
                            on_delete: OnDeleteStrategy::None,
                        }),
                        database_name: None,
                        default_value: None,
                        is_unique: false,
                        id_info: None,
                        scalar_list_strategy: None,
                        documentation: None,
                        is_generated: false,
                        is_updated_at: false,
                    },
                ],
                is_generated: false,
            },
        ],
        enums: vec![],
    };

    let schema = DatabaseSchema {
        tables: vec![
            Table {
                name: "City".to_string(),
                columns: vec![
                    Column {
                        name: "id".to_string(),
                        tpe: ColumnType {
                            raw: "integer".to_string(),
                            family: ColumnTypeFamily::Int,
                        },
                        arity: ColumnArity::Required,
                        default: None,
                        auto_increment: true,
                    },
                    Column {
                        name: "name".to_string(),
                        tpe: ColumnType {
                            raw: "text".to_string(),
                            family: ColumnTypeFamily::String,
                        },
                        arity: ColumnArity::Required,
                        default: None,
                        auto_increment: false,
                    },
                ],
                indices: vec![],
                primary_key: Some(PrimaryKey {
                    columns: vec!["id".to_string()],
                }),
                foreign_keys: vec![],
            },
            Table {
                name: "User".to_string(),
                columns: vec![
                    Column {
                        name: "id".to_string(),
                        tpe: ColumnType {
                            raw: "integer".to_string(),
                            family: ColumnTypeFamily::Int,
                        },
                        arity: ColumnArity::Required,
                        default: None,
                        auto_increment: true,
                    },
                    Column {
                        name: "city-id".to_string(),
                        tpe: ColumnType {
                            raw: "integer".to_string(),
                            family: ColumnTypeFamily::Int,
                        },
                        arity: ColumnArity::Required,
                        default: None,
                        auto_increment: false,
                    },
                    Column {
                        name: "city-name".to_string(),
                        tpe: ColumnType {
                            raw: "text".to_string(),
                            family: ColumnTypeFamily::String,
                        },
                        arity: ColumnArity::Required,
                        default: None,
                        auto_increment: false,
                    },
                ],
                indices: vec![],
                primary_key: None,
                foreign_keys: vec![ForeignKey {
                    columns: vec!["city-id".to_string(), "city-name".to_string()],
                    referenced_table: "City".to_string(),
                    on_delete_action: ForeignKeyAction::NoAction,
                    referenced_columns: vec!["id".to_string(), "name".to_string()],
                }],
            },
        ],
        enums: vec![],
        sequences: vec![],
    };
    let data_model = calculate_model(&schema).expect("calculate data model");

    assert_eq!(data_model, ref_data_model);
}
