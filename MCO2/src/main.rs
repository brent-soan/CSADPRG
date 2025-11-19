use polars::prelude::*;
use serde_json::json;
use std::fs::File;
use std::io;
use std::io::Write;

fn main() { 
    let mut user_input;
    let mut df: DataFrame = Default::default();
    let mut is_dataset_loaded = false;

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
            is_dataset_loaded = true;
        } else if user_input == "2" { 
            if is_dataset_loaded {
                generate_reports(&df);
            } else {
                println!("ERROR: Dataset is not yet loaded.");
            }
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
    println!("Loaded {} rows ", df.shape().0);
    
    // Remove rows with invalid values in approved_budget_for_contract and contract_cost
    let temp = df.clone()
        .lazy()
        .select([
            col("project_id"),
            col("approved_budget_for_contract").cast(DataType::Float64),
            col("contract_cost").cast(DataType::Float64)
        ])
        .filter(is_not_null(col("approved_budget_for_contract")).and(is_not_null(col("contract_cost"))))
        .collect()
        .unwrap();
    println!("Removed {} rows with invalid values", df.shape().0 - temp.shape().0);

    *df = df.clone()
        .lazy()
        .select([
            all().exclude_cols(["approved_budget_for_contract", "contract_cost"]).as_expr()
        ])
        .join(
            temp.clone().lazy(),
            [col("project_id")],
            [col("project_id")],
            JoinArgs::new(JoinType::Inner)
        )
        .collect()
        .unwrap();
    
    *df = df.clone()
        .lazy()
        .filter(
            col("start_date").dt().year().eq(lit(2021)).or(col("start_date").dt().year().eq(lit(2022))).or(col("start_date").dt().year().eq(lit(2023)))
        )
        .collect()
        .unwrap();
    println!("Filtered {} rows for 2021-2023", df.shape().0);
   
   // Add columns 
    *df = df.clone()
        .lazy()
        .with_columns([
            (col("approved_budget_for_contract") - col("contract_cost")).alias("cost_savings"),
            (col("actual_completion_date") - col("start_date")).dt().total_days(false).alias("completion_delay_days")
        ])
        .collect()
        .unwrap();
    println!("Added cost_savings and completion_delay_days");
    
    println!("Loading finished");
}

fn generate_reports(df: &DataFrame) {
    let summary = json!({
        "total_projects": df.shape().0,
    });
    
    println!("\nGenerating reports...");
    println!("Report 1: Regional Flood Mitigation Efﬁciency Summary");
    let mut report1_df = df.clone()
        .lazy()
        .group_by([
            col("region"),
            col("main_island")])
        .agg([
            col("approved_budget_for_contract").sum().alias("total_budget"),
            col("cost_savings").median().alias("median_savings"),
            col("completion_delay_days").mean().alias("average_delay"),
            (col("completion_delay_days").gt(lit(30)).count() / col("completion_delay_days").count() * lit(100)).alias("high_delay_percent"),
        ])
        .with_columns([
            (col("median_savings") / col("average_delay") * lit(100)).alias("efficiency_score")
        ])
        .sort(["efficiency_score"], SortMultipleOptions::new()
            .with_order_descending(true)
        )
        .collect()
        .unwrap();
    println!("{report1_df}");
    
    let mut report1_file = std::fs::File::create("reports/report1.csv").unwrap();
    CsvWriter::new(&mut report1_file).finish(&mut report1_df).unwrap();
    println!("Full table exported to report1.csv");
        
    println!("\nReport 2: Top Contractors Performance Ranking");
    let mut report2_df = df.clone()
        .lazy()
        .group_by([
            col("contractor")
        ])
        .agg([
            col("contract_cost").sum().alias("total_cost"),
            len().alias("total_projects"),
            col("completion_delay_days").mean().alias("average_delay"),
            col("cost_savings").sum().alias("total_savings"),
        ])
        .filter(
            col("total_projects").gt(lit(4)) 
        )
        .sort(["total_cost"], SortMultipleOptions::new()
            .with_order_descending(true)
        )
        .collect()
        .unwrap()
        .head(Some(15))
        .lazy()
        .with_column(
            ((lit(1) - col("average_delay") / lit(90)) * (col("total_savings") / col("total_cost")) * lit(100)).alias("reliability_index"),
        )
        .with_columns([
            when(col("reliability_index").lt(lit(50))).then(lit("High Risk")).otherwise(lit("Low Risk")).alias("risk_flag"),
            lit(Series::new("rank".into(), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]))
        ])
        .collect()
        .unwrap();
    
    println!("{report2_df}");
    let mut report2_file = std::fs::File::create("reports/report2.csv").unwrap();
    CsvWriter::new(&mut report2_file).finish(&mut report2_df).unwrap();
    println!("Full table exported to report2.csv");
    
    println!("\nReport 3: Annual Project Type Cost Overrun Trends");
    let baseline = df.clone()
        .lazy()
        .filter(col("funding_year").eq(lit(2021)))
        .group_by([col("work_type")])
        .agg([ col("cost_savings").mean().alias("baseline_avg_savings") ]);
    
    let mut report3_df = df.clone()
        .lazy()
        .group_by([
            col("funding_year"),
            col("work_type")
        ])
        .agg([
            len().alias("total_projects"),
            col("cost_savings").mean().alias("average_cost_savings"),
            (col("cost_savings").lt(lit(0)).cast(DataType::Float64).sum() / len() * lit(100)).alias("overrun_rate"),
        ])
        .join(
            baseline,
            [ col("work_type") ],
            [ col("work_type") ],
            JoinArgs::new(JoinType::Left),
        )
        .with_column(
            when(col("baseline_avg_savings").is_null().or(col("baseline_avg_savings").eq(lit(0.0))))
                .then(lit(0))
                .otherwise(
                    (col("average_cost_savings") - col("baseline_avg_savings"))
                        / col("baseline_avg_savings")
                        * lit(100.0)
                )
                .alias("year_over_year_change")
        )
        .select([
            all().exclude_cols(["baseline_avg_savings"]).as_expr()
        ])
        .sort(["average_cost_savings"], SortMultipleOptions::new()
            .with_order_descending(true)
        )
        .collect()
        .unwrap();
    println!("{report3_df}");
    let mut report3_file = std::fs::File::create("reports/report3.csv").unwrap();
    CsvWriter::new(&mut report3_file).finish(&mut report3_df).unwrap();
    println!("Full table exported to report3.csv");

    let mut f = File::create("reports/summary.json").unwrap();
    f.write_all(serde_json::to_string_pretty(&summary).unwrap().as_bytes()).unwrap();
    println!("Summary exported to summary.json");
    
    println!("Reports generated");
}