use polars::prelude::*;

fn main() { 
    let schema = Schema::from_iter(vec![
        Field::new("MainIsland".into(), DataType::String),
        Field::new("Region".into(), DataType::String),
        Field::new("Province".into(), DataType::String),
        Field::new("LegislativeDistrict".into(), DataType::String),
        Field::new("Municipality".into(), DataType::String),
        Field::new("DistrictEngineeringOffice".into(), DataType::String),
        Field::new("ProjectId".into(), DataType::String),
        Field::new("ProjectName".into(), DataType::String),
        Field::new("TypeOfWork".into(), DataType::String),
        Field::new("FundingYear".into(), DataType::Int32),
        Field::new("ContractId".into(), DataType::String),
        Field::new("ApprovedBudgetForContract".into(), DataType::String),
        Field::new("ContractCost".into(), DataType::String),
        Field::new("ActualCompletionDate".into(), DataType::Date),
        Field::new("Contractor".into(), DataType::String),
        Field::new("ContractorCount".into(), DataType::Int32),
        Field::new("StartDate".into(), DataType::Date),
        Field::new("ProjectLatitude".into(), DataType::Float64),
        Field::new("ProjectLongitude".into(), DataType::Float64),
        Field::new("ProvincialCapital".into(), DataType::String),
        Field::new("ProvincialCapitalLatitude".into(), DataType::Float64),
        Field::new("ProvincialCapitalLongitude".into(), DataType::Float64),
    ]);

    let df = LazyCsvReader::new(PlPath::new("dpwh_flood_control_projects.csv"))
        .with_has_header(true)
        .with_schema(Arc::new(schema).into())
        .finish()
        .unwrap()
        .select([
            col("MainIsland").alias("main_island"),
            col("Region").alias("region"),
            col("Province").alias("province"),
            col("LegislativeDistrict").alias("legislative_district"),
            col("Municipality").alias("municipality"),
            col("DistrictEngineeringOffice").alias("district_engineering_office"),
            col("ProjectId").alias("project_id"),
            col("ProjectName").alias("project_name"),
            col("TypeOfWork").alias("type_of_work"),
            col("FundingYear").alias("funding_year"),
            col("ContractId").alias("contract_id"),
            col("ApprovedBudgetForContract").alias("approved_budget_for_contract"),
            col("ContractCost").alias("contract_cost"),
            col("ActualCompletionDate").alias("actual_completion_date"),
            col("Contractor").alias("contractor"),
            col("ContractorCount").alias("contractor_count"),
            col("StartDate").alias("start_date"),
            col("ProjectLatitude").alias("project_latitude"),
            col("ProjectLongitude").alias("project_longitude"),
            col("ProvincialCapital").alias("provincial_capital"),
            col("ProvincialCapitalLatitude").alias("provincial_capital_latitude"),
            col("ProvincialCapitalLongitude").alias("provincial_capital_longitude"),
        ])
        .collect()
        .unwrap();

    println!("{}", df.head(Some(3)));
}