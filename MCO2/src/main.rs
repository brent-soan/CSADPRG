use polars::prelude::*;
use std::io;
use std::io::Write;

fn main() { 
    let mut user_input;
    let mut df: DataFrame = Default::default();

    println!("Welcome to CSADPRG MCO2 Data Analysis Pipeline Project made with Rust!");
    
    loop {
        user_input = input("\nMain Menu
[0] Exit
[1] Load dataset
[2] Generate reports
Select option");

        if user_input == "0" {
            break;
        } else if user_input == "1" { 
            load_dataset(&mut df);
        } else if user_input == "2" { 
            generate_reports(&df);
        } else {
            println!("ERROR: Input not valid.");
        }
    }

    println!("\nGood bye!");
}

fn input(prompt: &str) -> String {
    let mut user_input = String::new();
    
    print!("{}: ", prompt);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut user_input).expect("ERROR: Input failed.");

    user_input.trim().to_string()
}

fn load_dataset(df: &mut DataFrame) {
    println!("\nLoading dataset...");

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

    *df = LazyCsvReader::new(PlPath::new("dpwh_flood_control_projects.csv"))
        .with_has_header(true)
        .with_schema(Arc::new(schema).into())
        .finish()
        .unwrap() // Extract LazyFrame
        .select([ // Rename columns
            col("MainIsland").alias("main_island"),
            col("Region").alias("region"),
            col("Province").alias("province"),
            col("LegislativeDistrict").alias("legislative_district"),
            col("Municipality").alias("municipality"),
            col("DistrictEngineeringOffice").alias("district_engineering_office"),
            col("ProjectId").alias("project_id"),
            col("ProjectName").alias("project_name"),
            col("TypeOfWork").alias("work_type"),
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
        .unwrap(); // Extract DataFrame
    println!("{} rows loaded", df.shape().0);
    
    let temp = df.clone()
        .lazy()
        .select([
            col("approved_budget_for_contract").cast(DataType::Float64),
            col("contract_cost").cast(DataType::Float64)
        ])
        .filter(is_not_null(col("approved_budget_for_contract")).and(is_not_null(col("contract_cost"))))
        .collect()
        .unwrap();
    println!("{}", temp.shape().0);
    
    *df = df.clone()
        .lazy()
        .filter(
            col("start_date").dt().year().eq(lit(2021)).or(col("start_date").dt().year().eq(lit(2022))).or(col("start_date").dt().year().eq(lit(2023)))
        )
        .collect()
        .unwrap();
    println!("{} filtered for 2021-2023", df.shape().0);
   
   // Add columns 
    *df = df.clone()
            .lazy()
            .with_columns([
                (col("approved_budget_for_contract") - col("contract_cost")).alias("cost_savings"),
                (col("actual_completion_date") - col("start_date")).dt().total_days().alias("completion_delay_days")
            ])
            .collect()
            .unwrap();
    
    println!("Loading finished");
}

fn generate_reports(df: &DataFrame) {
    println!("\nGenerating reports...");
    println!("Reports generated");
}