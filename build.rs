fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .field_attribute("id", "#[serde(rename = \"_id\")]")
        .field_attribute(
            "id",
            "#[serde(serialize_with = \"serialize_hex_string_as_object_id\")]",
        )
        .field_attribute(
            "id",
            "#[serde(deserialize_with = \"deserialize_object_id_as_hex_string\")]",
        )
        .type_attribute(".", "#[derive(Deserialize, Serialize)]")
        .type_attribute(".", "#[serde(rename_all = \"camelCase\")]")
        .compile(&["proto/todo.proto"], &["proto/"])?;
    Ok(())
}
