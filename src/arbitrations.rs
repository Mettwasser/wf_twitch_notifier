use arbitration_data::model::{
    dict::LanguageDict,
    regions::ExportRegions,
};

pub fn load_arbi_data() -> anyhow::Result<arbitration_data::ArbitrationData> {
    let arbi_time_node_mapping = csv::Reader::from_reader(include_str!("../arbys.csv").as_bytes());
    let export_regions: ExportRegions<'_> = serde_json::from_str(include_str!("../regions.json"))?;
    let language_dict: LanguageDict = serde_json::from_str(include_str!("../dict.en.json"))?;

    let arbi_data = arbitration_data::ArbitrationData::new(
        arbi_time_node_mapping,
        export_regions,
        language_dict,
    )?;

    Ok(arbi_data)
}
